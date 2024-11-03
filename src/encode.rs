//! Base32 encoding routines
use crate::alphabet::{Alphabet, ZBASE32, EncodeOrder};

#[cfg(any(feature = "alloc", feature = "std", test))]
use std::{string::String, vec};

///Returns encoded length for given input length
pub fn encoded_len(bytes_len: usize) -> Option<usize> {
    let min_bytes = bytes_len / 5;
    let rem = bytes_len % 5;
    min_bytes.checked_mul(8).and_then(|c| c.checked_add(rem * 2 + 1))
}

///Encode base32 using the specified [Alphabet] and the predefined output slice.
///Returns a `usize` of how many output bytes are filled.
pub fn encode_alphabet_slice<T: AsRef<[u8]>>(
    input: T,
    output_buf: &mut [u8],
    alphabet: &Alphabet,
) -> usize {
    let encode_table = alphabet.encode_symbols;
    let input_bytes = input.as_ref();
    let mut remain = -1_i32;
    let mut o = 0_usize;

    if alphabet.encode_order == EncodeOrder::OrderInversed {
        for i in 0..input_bytes.len() {
            remain = match i % 5 {
                0 => {
                    // 8 bits of input and 3 to remain
                    let x = input_bytes[i] as i32;
                    output_buf[o] = encode_table[(x & 0x1F) as usize];
                    o = o + 1;
                    x >> 5
                },
                1 => {
                    // 11 bits of input, 1 to remain
                    let inp = input_bytes[i] as i32;
                    let x = remain | inp << 3;
                    output_buf[o] = encode_table[(x & 0x1F) as usize];
                    o = o + 1;
                    output_buf[o] = encode_table[(x >> 5 & 0x1F) as usize];
                    o = o + 1;
                    x >> 10
                }
                2 => {
                    // 9 bits of input, 4 to remain
                    let inp = input_bytes[i] as i32;
                    let x = remain | inp << 1;
                    output_buf[o] = encode_table[(x & 0x1F) as usize];
                    o = o + 1;
                    x >> 5
                },
                3 => {
                    // 12 bits of input, 2 to remain
                    let inp = input_bytes[i] as i32;
                    let x = remain | inp << 4;
                    output_buf[o] = encode_table[(x & 0x1F) as usize];
                    o = o + 1;
                    output_buf[o] = encode_table[(x >> 5 & 0x1F) as usize];
                    o = o + 1;
                    x >> 10 & 0x3
                },
                4 => {
                    // 10 bits of output, nothing to remain
                    let inp = input_bytes[i] as i32;
                    let x = remain | inp << 2;
                    output_buf[o] = encode_table[(x & 0x1F) as usize];
                    o = o + 1;
                    output_buf[o] = encode_table[(x >> 5 & 0x1F) as usize];
                    o = o + 1;
                    -1
                },
                _ => unreachable!("Impossible remainder"),
            };
        }
    }
    else {
        for i in 0..input_bytes.len() {
            remain = match i % 5 {
                0 => {
                    // 8 bits of input and 3 to remain
                    let inp = input_bytes[i] as i32;
                    let x = inp >> 3;
                    output_buf[o] = encode_table[(x & 0x1F) as usize];
                    o = o + 1;
                    (inp & 7) << 2
                },
                1 => {
                    // 11 bits of input, 1 to remain
                    let inp = input_bytes[i] as i32;
                    let x = (remain << 6) | inp;
                    output_buf[o] = encode_table[(x >> 6 & 0x1F) as usize];
                    o = o + 1;
                    output_buf[o] = encode_table[(x >> 1 & 0x1F) as usize];
                    o = o + 1;
                    (x & 0x1) << 4
                }
                2 => {
                    // 9 bits of input, 4 to remain
                    let inp = input_bytes[i] as i32;
                    let x = (remain << 4) | inp;
                    output_buf[o] = encode_table[(x >> 4 & 0x1F) as usize];
                    o = o + 1;
                    (x & 15) << 1
                },
                3 => {
                    // 12 bits of input, 2 to remain\
                    let inp = input_bytes[i] as i32;
                    let x = remain << 7 | inp;
                    output_buf[o] = encode_table[(x >> 7 & 0x1F) as usize];
                    o = o + 1;
                    output_buf[o] = encode_table[(x >> 2 & 0x1F) as usize];
                    o = o + 1;
                    (x & 3) << 3
                },
                4 => {
                    // 10 bits of output, nothing to remain
                    let inp = input_bytes[i] as i32;
                    let x = remain << 5 | inp;
                    output_buf[o] = encode_table[(x >> 5 & 0x1F) as usize];
                    o = o + 1;
                    output_buf[o] = encode_table[(x & 0x1F) as usize];
                    o = o + 1;
                    -1
                },
                _ => unreachable!("Impossible remainder"),
            };
        }
    }

    if remain >= 0 {
        output_buf[o] = encode_table[(remain & 0x1F) as usize];
        o = o + 1;
    }

    o
}

///Encode base32 using the specified [Alphabet].
///Returns a `String`.
///
///# Example
///
///```rust
///extern crate base32;
///
///fn main() {
///    let encoded = base32::encode_alphabet(
///        "hello",
///        &base32::alphabet::RFC,
///    );
///    println!("{}", encoded);
///    // Prints 'NBSWY3DP'
///}
///```
#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn encode_alphabet<T: AsRef<[u8]>>(input: T, alphabet: &Alphabet) -> String {
    let encoded_size = encoded_len(input.as_ref().len())
        .expect("usize overflow when calculating buffer size");
    let mut buf = vec![0; encoded_size];
    let enc_len = encode_alphabet_slice(input, &mut buf[..], alphabet);
    String::from_utf8(buf[0..enc_len].to_owned()).expect("Invalid UTF8")
}

///Encode base32 using the default alphabet
///Returns a `String` result
///
///# Example
///
///```rust
///extern crate base32;
///
///fn main() {
///    let encoded = base32::encode("hello");
///    println!("{}", encoded);
///    // Prints 'em3ags7p'
///}
///```
#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    encode_alphabet(input, &ZBASE32)
}

#[cfg(test)]
mod tests {
    use crate::encode::*;
    use crate::alphabet::*;

    #[test]
    fn simple_encode_zbase() {
        assert_eq!(
            "wm3g84fg13cy",
            encode("test123"),
        );
        assert_eq!(
            "em3ags7p",
            encode("hello"),
        );
    }
    #[test]
    fn empty_encode_zbase() {
        assert_eq!(
            "",
            encode(""),
        );
    }
    #[test]
    fn series_encode_zbase() {
        assert_eq!(
            "bd",
            encode("a"),
        );
        assert_eq!(
            "bmay",
            encode("aa"),
        );
        assert_eq!(
            "bmang",
            encode("aaa"),
        );
        assert_eq!(
            "bmansob",
            encode("aaaa"),
        );
        assert_eq!(
            "bmansofc",
            encode("aaaaa"),
        );
        assert_eq!(
            "bmansofcbd",
            encode("aaaaaa"),
        );
        assert_eq!(
            "bmansofcbmay",
            encode("aaaaaaa"),
        );
        assert_eq!(
            "bmansofcbmang",
            encode("aaaaaaaa"),
        );
    }
    #[test]
    fn simple_encode_rfc() {
        assert_eq!(
            "ORSXG5BRGIZQ",
            encode_alphabet("test123", &RFC),
        );
        assert_eq!(
            "NBSWY3DP",
            encode_alphabet("hello", &RFC),
        );
    }
    #[test]
    fn empty_encode_rfc() {
        assert_eq!(
            "",
            encode_alphabet("", &RFC),
        );
    }
    #[test]
    fn series_encode_rfc() {
        assert_eq!(
            "ME",
            encode_alphabet("a", &RFC),
        );
        assert_eq!(
            "MFQQ",
            encode_alphabet("aa", &RFC),
        );
        assert_eq!(
            "MFQWC",
            encode_alphabet("aaa", &RFC),
        );
        assert_eq!(
            "MFQWCYI",
            encode_alphabet("aaaa", &RFC),
        );
        assert_eq!(
            "MFQWCYLB",
            encode_alphabet("aaaaa", &RFC),
        );
        assert_eq!(
            "MFQWCYLBME",
            encode_alphabet("aaaaaa", &RFC),
        );
        assert_eq!(
            "MFQWCYLBMFQQ",
            encode_alphabet("aaaaaaa", &RFC),
        );
        assert_eq!(
            "MFQWCYLBMFQWC",
            encode_alphabet("aaaaaaaa", &RFC),
        );
    }
}