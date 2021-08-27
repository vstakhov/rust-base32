use crate::alphabet::{Alphabet, ZBASE32, EncodeOrder};

#[cfg(any(feature = "alloc", feature = "std", test))]
use std::{string::String, vec};


pub fn encoded_len(bytes_len: usize) -> Option<usize> {
    let min_bytes = bytes_len / 5;
    let rem = bytes_len % 5;
    min_bytes.checked_mul(8).and_then(|c| c.checked_add(rem * 2 + 1))
}

pub fn encode_alphabet_slice<T: AsRef<[u8]>>(
    alphabet: &Alphabet,
    input: T,
    output_buf: &mut [u8],
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

#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn encode_with_alphabet<T: AsRef<[u8]>>(alphabet: &Alphabet, input: T) -> String {
    let encoded_size = encoded_len(input.as_ref().len())
        .expect("usize overflow when calculating buffer size");
    let mut buf = vec![0; encoded_size];
    let enc_len = encode_alphabet_slice(alphabet, input, &mut buf[..]);
    String::from_utf8(buf[0..enc_len].to_owned()).expect("Invalid UTF8")
}

#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    encode_with_alphabet(&ZBASE32, input)
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
            encode_with_alphabet(&RFC, "test123"),
        );
        assert_eq!(
            "NBSWY3DP",
            encode_with_alphabet(&RFC, "hello"),
        );
    }
    #[test]
    fn empty_encode_rfc() {
        assert_eq!(
            "",
            encode_with_alphabet(&RFC, ""),
        );
    }
    #[test]
    fn series_encode_rfc() {
        assert_eq!(
            "ME",
            encode_with_alphabet(&RFC, "a"),
        );
        assert_eq!(
            "MFQQ",
            encode_with_alphabet(&RFC, "aa"),
        );
        assert_eq!(
            "MFQWC",
            encode_with_alphabet(&RFC, "aaa"),
        );
        assert_eq!(
            "MFQWCYI",
            encode_with_alphabet(&RFC, "aaaa"),
        );
        assert_eq!(
            "MFQWCYLB",
            encode_with_alphabet(&RFC, "aaaaa"),
        );
        assert_eq!(
            "MFQWCYLBME",
            encode_with_alphabet(&RFC, "aaaaaa"),
        );
        assert_eq!(
            "MFQWCYLBMFQQ",
            encode_with_alphabet(&RFC, "aaaaaaa"),
        );
        assert_eq!(
            "MFQWCYLBMFQWC",
            encode_with_alphabet(&RFC, "aaaaaaaa"),
        );
    }
}