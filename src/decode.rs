use crate::alphabet::{Alphabet, ZBASE32, EncodeOrder};

#[cfg(any(feature = "alloc", feature = "std", test))]
use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error;

/// Potential decoding errors
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// An invalid byte was found in the input. The offset and offending byte are provided.
    InvalidByte(usize, u8),
    /// The length of the input is invalid.
    InvalidLength(usize),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecodeError::InvalidByte(index, byte) => {
                write!(f, "Invalid byte {}, offset {}.", byte, index)
            }
            DecodeError::InvalidLength(sz) => write!(f, "Encoded text cannot have a 5-bit remainder: length = {}", sz),
        }
    }
}

#[cfg(any(feature = "std", test))]
impl error::Error for DecodeError {
    fn description(&self) -> &str {
        match *self {
            DecodeError::InvalidByte(_, _) => "invalid byte",
            DecodeError::InvalidLength(_) => "invalid length",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

///Decode base32 using the default alphabet
///Returns a `Result` containing a `Vec<u8>`.
///
///# Example
///
///```rust
///extern crate base32;
///
///fn main() {
///    let bytes = base32::decode("em3ags7p").unwrap();
///    println!("{:?}", bytes);
///    // Prints 'hello'
///}
///```
#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    decode_alphabet(input, &ZBASE32)
}

///Decode from string reference as octets using the specified [Alphabet].
///Returns a `Result` containing a `Vec<u8>`.
///
///# Example
///
///```rust
///extern crate base32;
///
///fn main() {
///    let bytes = base32::decode_alphabet(
///        "NBSWY3DP",
///        &base32::alphabet::RFC,
///    ).unwrap();
///    println!("{:?}", bytes);
///    // Prints 'hello'
///}
///```
#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn decode_alphabet<T: AsRef<[u8]>>(
    input: T,
    alphabet: &Alphabet,
) -> Result<Vec<u8>, DecodeError> {
    let mut buffer = Vec::<u8>::with_capacity(
        decoded_len(input.as_ref().len()).expect("integer multiplication overflow"));

    decode_alphabet_vec(input, &mut buffer, &alphabet).map(|_| buffer)
}

///Decode from string reference as octets.
///Writes into the supplied `Vec`, which may allocate if its internal buffer isn't big enough.
///Returns a `Result` containing an empty tuple, aka `()`.
///
///# Example
///
///```rust
///extern crate base32;
///
///
///fn main() {
///    let mut buffer = Vec::<u8>::new();
///    // with the default engine
///    base32::decode_alphabet_vec(
///        "em3ags7p",
///        &mut buffer,
///        &base32::alphabet::ZBASE32
///    ).unwrap();
///    println!("{:?}", buffer);
///}
///```
#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn decode_alphabet_vec<T: AsRef<[u8]>>(
    input: T,
    buffer: &mut Vec<u8>,
    alphabet: &Alphabet,
) -> Result<(), DecodeError> {
    let input_bytes = input.as_ref();

    let estimate = decoded_len(input_bytes.len()).expect("integer multiplication overflow");
    buffer.resize(estimate, 0);

    let mut processed_bits = 0;
    let mut acc = 0_u32;
    let mut o = 0_usize;
    let mut i = o;

    if alphabet.encode_order == EncodeOrder::OrderInversed {
        for c in input_bytes {
            if processed_bits >= 8 {
                // Emit from left to right
                processed_bits -= 8;
                buffer[o] = (acc & 0xFF) as u8;
                o = o + 1;
                acc = acc >> 8;
            }
            let decoded = alphabet.decode_bytes[*c as usize];
            if decoded == 0xff {
                return Err(DecodeError::InvalidByte(i, *c));
            }

            acc = ((decoded as u32) << processed_bits) | acc;
            processed_bits = processed_bits + 5;
            i = i + 1;
        }
        if processed_bits > 0 {
            buffer[o] = (acc & 0xFF) as u8;
            o = o + 1;
        }
    }
    else {
        for c in input_bytes {
            let decoded = alphabet.decode_bytes[*c as usize];
            if decoded == 0xff {
                return Err(DecodeError::InvalidByte(i, *c));
            }

            acc = (acc << 5) | decoded as u32;
            processed_bits = processed_bits + 5;

            if processed_bits >= 8 {
                processed_bits = processed_bits - 8;
                // Emit from right to left
                buffer[o] = ((acc >> processed_bits) & 0xFF) as u8;
                o = o + 1;
                acc = acc & ((1 << processed_bits) - 1);
            }

            i = i + 1;
        }
    }

    buffer.resize(o, 0);

    Ok(())
}

fn decoded_len(bytes_len : usize) -> Option<usize> {
    let full_chunks = bytes_len / 8;
    let remainder = bytes_len % 8;
    full_chunks.checked_mul(5).and_then(|c| c.checked_add(remainder))
}

#[cfg(test)]
mod tests {
    use crate::encode::*;
    use crate::decode::*;
    use crate::alphabet::*;

    #[test]
    fn simple_decode_zbase() {
        assert_eq!(
            "test123".as_bytes(),
            decode("wm3g84fg13cy").expect("undecoded!"),
        );
        assert_eq!(
            "hello".as_bytes(),
            decode("em3ags7p").expect("undecoded"),
        );
    }

    #[test]
    fn simple_encode_decode_zbase() {
        assert_eq!("test123".as_bytes(),
            decode(encode("test123")).expect("undecoded"));
    }

    #[test]
    fn simple_decode_rfc() {
        assert_eq!(
            "test123".as_bytes(),
            decode_alphabet("ORSXG5BRGIZQ", &RFC).expect("undecoded!"),
        );
        assert_eq!(
            "hello".as_bytes(),
            decode_alphabet("NBSWY3DP", &RFC).expect("undecoded"),
        );
    }

    #[test]
    fn simple_encode_decode_rfc() {
        assert_eq!("test123".as_bytes(),
                   decode_alphabet(encode_alphabet("test123", &RFC),
                                   &RFC).expect("undecoded"));
    }
}
