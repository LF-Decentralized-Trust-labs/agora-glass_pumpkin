//! Error structs

use crate::common::MIN_BIT_LENGTH;
use core::result;
use thiserror::Error;

/// Default result struct
pub type Result<T = num_bigint::BigUint> = result::Result<T, Error>;

/// Error struct
#[derive(Debug, Error)]
pub enum Error {
    /// Handles when the bit sizes are too small
    #[error("The given bit length is too small; must be at least {MIN_BIT_LENGTH}: {0}")]
    BitLength(usize),

    /// Handles failures when accessing operating-system randomness.
    #[cfg(feature = "getrandom")]
    #[error("Failed to access operating-system randomness: {0}")]
    Random(#[from] getrandom::Error),
}
