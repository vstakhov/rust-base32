//! Provides Alphabet used for Base32 encoding and decoding
use std::{error, fmt};

pub const ALPHABET_SIZE: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EncodeOrder {
    OrderInversed,
    OrderNormal
}

/// Defines alphabet - 32 characters used for Base32
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Alphabet {
    pub(crate) encode_symbols: [u8; ALPHABET_SIZE],
    pub(crate) decode_bytes: [u8; u8::MAX as usize],
    pub(crate) encode_order : EncodeOrder,
}

impl Alphabet {
    /// Performs no checks so that it can be const.
    /// Used only for known-valid strings.
    const fn from_str_unsafe(alphabet: &str, encode_order: EncodeOrder) -> Self {
        let mut symbols = [0_u8; ALPHABET_SIZE];
        let source_bytes = alphabet.as_bytes();
        let mut decode_bytes = [0xff_u8; u8::MAX as usize];

        let mut index = 0;
        while index < ALPHABET_SIZE {
            let sym = source_bytes[index];
            symbols[index] = sym;
            decode_bytes[sym as usize] = index as u8;
            index = index + 1;
        }
        Alphabet { encode_symbols: symbols, decode_bytes, encode_order }
    }

    /// Checks input for printability and duplicates
    pub const fn from_str_order(alphabet: &str, encode_order: EncodeOrder) -> Result<Self, ParseAlphabetError> {
        const FIRST_PRINTABLE: u8 = 32;
        const LAST_PRINTABLE: u8 = 126;
        const DUPS_SIZE: usize = (LAST_PRINTABLE - FIRST_PRINTABLE) as usize;
        let source_bytes = alphabet.as_bytes();
        let mut dups : [bool; DUPS_SIZE] = [false; DUPS_SIZE];

        if source_bytes.len() != ALPHABET_SIZE {
            return Err(ParseAlphabetError::InvalidLength);
        }

        let mut index = 0;
        while index < ALPHABET_SIZE {
            let byte = source_bytes[index];
            // Must be printable for sanity
            if !(byte >= FIRST_PRINTABLE as u8 && byte <= LAST_PRINTABLE as u8) {
                return Err(ParseAlphabetError::UnprintableByte(byte));
            }

            let dup_idx = (byte - FIRST_PRINTABLE) as usize;
            if dups[dup_idx] {
                return Err(ParseAlphabetError::DuplicatedByte(byte));
            }

            dups[dup_idx] = true;
            index = index + 1;
        }

        Ok(Self::from_str_unsafe(alphabet, encode_order))
    }

    pub const fn from_str(alphabet: &str) -> Result<Self, ParseAlphabetError> {
        Self::from_str_order(alphabet, EncodeOrder::OrderNormal)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseAlphabetError {
    /// Alphabets must be 64 ASCII bytes
    InvalidLength,
    /// All bytes must be unique
    DuplicatedByte(u8),
    /// All bytes must be printable (in the range `[32, 126]`).
    UnprintableByte(u8),
}

#[cfg(any(feature = "std", test))]
impl fmt::Display for ParseAlphabetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseAlphabetError::InvalidLength => write!(f, "Invalid length - must be {} bytes", ALPHABET_SIZE),
            ParseAlphabetError::DuplicatedByte(b) => write!(f, "Duplicated byte: {:#04x}", b),
            ParseAlphabetError::UnprintableByte(b) => write!(f, "Unprintable byte: {:#04x}", b),
        }
    }
}

#[cfg(any(feature = "std", test))]
impl error::Error for ParseAlphabetError {}

/// ZBase32 alphabet
/// http://philzimmermann.com/docs/human-oriented-base-32-encoding.txt
pub const ZBASE32: Alphabet = Alphabet::from_str_unsafe(
    "ybndrfg8ejkmcpqxot1uwisza345h769",
    EncodeOrder::OrderInversed,
);

/// Bech32 alphabet used for bitcoin
pub const BECH32: Alphabet = Alphabet::from_str_unsafe(
   "qpzry9x8gf2tvdw0s3jn54khce6mua7l",
   EncodeOrder::OrderNormal,
);

/// RFC 4648 base32
pub const RFC: Alphabet = Alphabet::from_str_unsafe(
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567",
    EncodeOrder::OrderNormal,
);

#[cfg(test)]
mod tests {
    use crate::alphabet::*;

    #[test]
    fn detects_duplicate_start() {
        assert_eq!(
            ParseAlphabetError::DuplicatedByte(b'A'),
            Alphabet::from_str("AACDEFGHIJKLMNOPQRSTUVWXYZ234567")
                .unwrap_err()
        );
    }

    #[test]
    fn detects_duplicate_end() {
        assert_eq!(
            ParseAlphabetError::DuplicatedByte(b'7'),
            Alphabet::from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ234577")
                .unwrap_err()
        );
    }

    #[test]
    fn detects_duplicate_middle() {
        assert_eq!(
            ParseAlphabetError::DuplicatedByte(b'Z'),
            Alphabet::from_str("ABCDEFGHIJKLMNOPQRSTUVWXZZ234567")
                .unwrap_err()
        );
    }

    #[test]
    fn detects_length() {
        assert_eq!(
            ParseAlphabetError::InvalidLength,
            Alphabet::from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567xxx")
                .unwrap_err()
        );
    }

    #[test]
    fn detects_unprintable() {
        // form feed
        assert_eq!(
            ParseAlphabetError::UnprintableByte(0xc),
            Alphabet::from_str(
                "\x0cBCDEFGHIJKLMNOPQRSTUVWXYZ234567"
            )
                .unwrap_err()
        );
    }

    #[test]
    fn same_as_unchecked() {
        assert_eq!(
            RFC,
            Alphabet::from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567")
                .unwrap()
        )
    }
}