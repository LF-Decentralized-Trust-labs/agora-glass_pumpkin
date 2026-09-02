//! Generates cryptographically secure prime numbers.

pub use crate::common::{
    gen_prime as from_rng, is_prime as check_with, is_prime_baillie_psw as strong_check_with,
};

#[cfg(feature = "getrandom")]
use crate::error::Result;

/// Constructs a new prime number with a size of `bit_length` bits.
///
/// This will initialize a `getrandom::SysRng` instance and call the
/// `from_rng()` function.
///
/// Note: the `bit_length` MUST be at least 128-bits.
///
/// # Errors
///
/// Returns an error when `bit_length` is too small or operating-system
/// randomness is unavailable.
#[cfg(feature = "getrandom")]
pub fn new(bit_length: usize) -> Result {
    crate::common::gen_prime_from_system(bit_length)
}

/// Test if number is prime by
///
/// 1- Trial division by first 2048 primes
/// 2- Perform log2(bitlength) + 5 rounds of Miller-Rabin
///    depending on the number of bits
///
/// # Errors
///
/// Returns an error when operating-system randomness is unavailable.
#[cfg(feature = "getrandom")]
pub fn check(candidate: &num_bigint::BigUint) -> Result<bool> {
    crate::common::is_prime_with_system(candidate)
}

/// Checks if a number is probably prime using the deterministic-base
/// Baillie-PSW test.
#[cfg(feature = "getrandom")]
pub fn strong_check(candidate: &num_bigint::BigUint) -> bool {
    crate::common::is_prime_baillie_psw_without_rng(candidate)
}

#[cfg(all(test, feature = "getrandom"))]
mod tests {
    use super::{check, new, strong_check};

    #[test]
    fn tests() {
        for bits in &[128, 256, 512, 1024] {
            let n = new(*bits).unwrap();
            assert!(check(&n).unwrap());
            assert!(strong_check(&n));
        }
    }
}
