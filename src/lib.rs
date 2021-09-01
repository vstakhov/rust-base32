
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

