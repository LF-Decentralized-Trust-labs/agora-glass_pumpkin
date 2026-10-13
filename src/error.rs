//! Errors returned during prime generation and checking.

use crate::common::MIN_BIT_LENGTH;
use core::result;
use thiserror::Error;

/// A result returned by fallible crate operations.
pub type Result<T = num_bigint::BigUint> = result::Result<T, Error>;

/// An error returned during prime generation or checking.
#[derive(Debug, Error)]
pub enum Error {
    /// The requested bit length is smaller than the supported minimum.
    #[error("The given bit length is too small; must be at least {MIN_BIT_LENGTH}: {0}")]
    BitLength(usize),

    /// Handles failures when accessing operating-system randomness.
    #[cfg(feature = "getrandom")]
    #[error("Failed to access operating-system randomness: {0}")]
    Random(#[from] getrandom::Error),
}
