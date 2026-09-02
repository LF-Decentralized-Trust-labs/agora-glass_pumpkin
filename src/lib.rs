#![deny(
    warnings,
    missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

//! A crate for generating large prime numbers, suitable for cryptography.
//!
//! Primes are generated similarly to OpenSSL except it applies some recommendations
//! from the [Prime and Prejudice](https://eprint.iacr.org/2018/749.pdf).
//!
//! 1. Generate a random odd number of a given bit-length.
//! 2. Divide the candidate by the first 2048 prime numbers
//! 3. Run the Baillie-PSW test.
//! 4. Run `log2(bits) + 4` additional Miller-Rabin tests.

extern crate alloc;

mod common;
pub mod error;
pub mod prime;
pub mod safe_prime;
