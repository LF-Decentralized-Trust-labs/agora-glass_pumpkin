# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-10-13

### Changed

- Updated the random-number APIs to `rand_core` 0.10. The `from_rng`,
  `check_with`, and `strong_check_with` functions now accept
  `rand_core::Rng` implementations instead of `rand_core::RngCore` 0.6
  implementations.
- Updated `num-bigint` from 0.4 to 0.5. Public functions returning or accepting
  `BigUint` now use the 0.5 type.
- Changed `prime::check` and `safe_prime::check` from returning `bool` to
  returning `Result<bool, glass_pumpkin::error::Error>` so operating-system
  randomness failures can be reported.
- Replaced `Error::OsRngInitialization(rand_core::Error)` with
  `Error::Random(getrandom::Error)`.
- Changed prime generation to use the conventional Baillie-PSW test followed
  by additional random-base Miller-Rabin rounds.
- Changed the crate license from Apache-2.0 to MIT OR Apache-2.0.
- Raised the minimum supported Rust version to 1.88.

### Added

- Added direct support for fallible operating-system randomness through
  `getrandom::SysRng`.
- Added a generic type parameter to `glass_pumpkin::error::Result`; it still
  defaults to `num_bigint::BigUint` for compatibility with generation APIs.

### Migration from 1.x

Handle the result returned by the system-randomness checking functions:

```rust
use glass_pumpkin::prime;
use num_bigint::BigUint;

let candidate = BigUint::from(5_u8);
let is_prime = prime::check(&candidate)?;
# assert!(is_prime);
# Ok::<(), glass_pumpkin::error::Error>(())
```

Applications supplying their own RNG must update it to an implementation of
`rand_core::Rng` 0.10. Applications that name `num_bigint::BigUint` in their
own API should likewise update to `num-bigint` 0.5.

[2.0.0]: https://github.com/LF-Decentralized-Trust-labs/agora-glass_pumpkin/compare/v1.9.1...v2.0.0
