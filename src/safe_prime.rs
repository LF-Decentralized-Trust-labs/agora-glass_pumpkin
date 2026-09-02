//! Generates cryptographically secure safe prime numbers.

pub use crate::common::{
    gen_safe_prime as from_rng, is_safe_prime as check_with,
    is_safe_prime_baillie_psw as strong_check_with,
};

#[cfg(feature = "getrandom")]
use crate::error::Result;

/// Constructs a new safe prime number with a size of `bit_length` bits.
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
    crate::common::gen_safe_prime_from_system(bit_length)
}

/// Checks if number is a safe prime.
///
/// # Errors
///
/// Returns an error when operating-system randomness is unavailable.
#[cfg(feature = "getrandom")]
pub fn check(candidate: &num_bigint::BigUint) -> Result<bool> {
    crate::common::is_safe_prime_with_system(candidate)
}

/// Checks if a number is probably a safe prime using the deterministic-base
/// Baillie-PSW test on both the number and its Sophie Germain prime.
#[cfg(feature = "getrandom")]
pub fn strong_check(candidate: &num_bigint::BigUint) -> bool {
    crate::common::is_safe_prime_baillie_psw_without_rng(candidate)
}

#[cfg(all(test, feature = "getrandom"))]
mod tests {
    use super::{check, new, strong_check};

    #[test]
    fn tests() {
        for bits in &[128, 256, 384] {
            let n = new(*bits).unwrap();
            assert!(check(&n).unwrap());
            assert!(strong_check(&n));
        }
    }
}
