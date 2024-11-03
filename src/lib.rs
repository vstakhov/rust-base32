//! # Rspamd Base32 encode/decode library
//! This is a conversion module from and to [zbase32](http://philzimmermann.com/docs/human-oriented-base-32-encoding.txt)
//! encoding. It also supports [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648)
//! and [Bech32](https://en.bitcoin.it/wiki/Bech32)  alphabets.
//!
//! The main purpose of zbase32 is to provide *human* readable encoding that is
//! more efficient than `hex` encoding.
//! `zbase32` utilizes up to `len * 5 / 8` of space for encoded date and contains
//! no padding (and hence no error control, like `base64`).
//! However, it seems to be much readable for a human when an encoding does not contain padding.
//!
//! ## Disclaimers
//!
//! This module is intended to be compatible with [Rspamd](https://rspamd.com)
//! base32 [encoding](https://rspamd.com/doc/lua/rspamd_util.html#f0372b),
//! so it has **bug-to-bug** compatibility with Rspamd C implementation including:
//!
//! - Zbase32 encodes data in reversed octets order (due to the initial bug in Rspamd and lack of test vectors)
//! - RFC 4648 encoding does not include padding (because padding as defined in RFC for base32 is just ugly)
//!
//!
//! ## Example
//!
//! ~~~rust
//!use rspamd_base32::{encode, decode};
//!
//!fn main() {
//!  let a = b"hello world";
//!  let b = "em3ags7py376g3tprd";
//!
//!  assert_eq!(encode(a), b);
//!  assert_eq!(a, &decode(b).unwrap()[..]);
//!}
//!~~~
#![forbid(unsafe_code)]

pub mod alphabet;
pub mod encode;
#[cfg(any(feature = "alloc", feature = "std", test))]
pub use crate::encode::{encode, encode_alphabet, encode_alphabet_slice};

pub mod decode;
#[cfg(any(feature = "alloc", feature = "std", test))]
pub use crate::decode::{decode, decode_alphabet, decode_alphabet_vec};

#[cfg(test)]
mod tests;

