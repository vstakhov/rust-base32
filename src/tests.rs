use crate::encode::*;
use crate::decode::*;
use crate::alphabet::*;
use rand::prelude::*;

fn roundtrip_random(
    alphabet: &Alphabet,
    byte_len: usize,
    max_rounds: u64,
) {
    // let the short ones be short but don't let it get too crazy large
    let mut r = rand::rngs::StdRng::from_entropy();
    let mut decode_buf : Vec<u8> = Vec::new();
    let mut byte_buf : Vec<u8> = Vec::new();

    for _ in 0..max_rounds {
        byte_buf.clear();
        decode_buf.clear();
        while byte_buf.len() < byte_len {
            byte_buf.push(r.gen::<u8>());
        }

        let encoded = encode_alphabet(&byte_buf, alphabet);
        decode_alphabet_vec(&encoded, &mut decode_buf, alphabet).unwrap();

        assert_eq!(byte_buf, decode_buf);
    }
}

fn compare_decode(expected: &str, target: &str) {
    assert_eq!(
        expected,
        String::from_utf8(decode(target).unwrap()).unwrap()
    );
    assert_eq!(
        expected,
        String::from_utf8(decode(target.as_bytes()).unwrap()).unwrap()
    );
}

#[test]
fn simple_decode_encode() {
    compare_decode("test", &encode(&decode(&encode(b"test")).unwrap()));
}

#[test]
fn encode_decode_random_zbase_small() {
    let alphabet = ZBASE32;
    for input_len in 0..40 {
        roundtrip_random(&alphabet, input_len, 10);
    }
}

#[test]
fn encode_decode_random_zbase_different() {
    let alphabet = ZBASE32;
    for input_len in [0,100,10,2,1024,32768,511,5,7,11,8] {
        roundtrip_random(&alphabet, input_len, 100);
    }
}

#[test]
fn encode_decode_random_rfc_small() {
    let alphabet = RFC;
    for input_len in 0..40 {
        roundtrip_random(&alphabet, input_len, 10);
    }
}

#[test]
fn encode_decode_random_bleach_small() {
    let alphabet = BLEACH32;
    for input_len in 0..40 {
        roundtrip_random(&alphabet, input_len, 10);
    }
}