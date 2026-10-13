# Glass Pumpkin

[![Build Status][build-image]][build-link]
[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![MIT OR Apache-2.0][license-badge]

A random number generator for generating large prime numbers, suitable for cryptography.

## Purpose

`glass_pumpkin` is a cryptographically-secure, random number generator, useful for generating large prime numbers.
This library is inspired by [pumpkin](https://github.com/zcdziura/pumpkin) except its meant to be used with rust stable.
It also lowers the 512-bit restriction to 128-bits so these can be generated and used for elliptic curve prime fields.
It exposes the prime testing functions as well.
This crate uses [num-bigint](https://crates.io/crates/num-bigint) instead of `ramp`. I have found
`num-bigint` to be just as fast as `ramp` for generating primes. On average, generating primes takes less
than 200ms and safe primes about 10 seconds on modern hardware.

## Installation

Add the following to your `Cargo.toml` file:

```toml
glass_pumpkin = "2.0"
```

Version 2.0 requires Rust 1.88 or newer. See the
[changelog](https://github.com/LF-Decentralized-Trust-labs/agora-glass_pumpkin/blob/main/CHANGELOG.md)
for breaking changes and migration guidance from 1.x.

## Features

The default features are `std` and `getrandom`:

- `getrandom` enables the `new`, `check`, and `strong_check` convenience APIs in
  the `prime` and `safe_prime` modules. The `new` and `check` functions obtain
  randomness from the operating system.
- `std` and `no_std` are compatibility markers. The library itself is always
  compiled with `#![no_std]` and uses `alloc`.

For a `no_std` consumer that supplies its own RNG, disable the default features:

```toml
glass_pumpkin = { version = "2.0", default-features = false, features = ["no_std"] }
```

Use `from_rng`, `check_with`, and `strong_check_with` when supplying an
implementation of `rand_core::Rng` directly.

## Example

```rust
use glass_pumpkin::prime;

fn main() -> Result<(), glass_pumpkin::error::Error> {
    let p = prime::new(1024)?;
    let q = prime::new(1024)?;

    let _n = p * q;

    Ok(())
}
```

You can also supply any RNG that implements `rand_core::Rng`.

```rust
use glass_pumpkin::prime;
use rand::rng;

fn main() -> Result<(), glass_pumpkin::error::Error> {
    let mut rng = rng();
    let p = prime::from_rng(1024, &mut rng)?;
    let q = prime::from_rng(1024, &mut rng)?;

    let _n = p * q;

    Ok(())
}
```

## Prime Generation

`Primes` are generated similarly to OpenSSL except it applies some recommendations from the [Prime and Prejudice](https://eprint.iacr.org/2018/749.pdf) paper and uses
the Baillie-PSW method:

1. Generate a random odd number of a given bit-length.
1. Divide the candidate by the first 2048 prime numbers. This helps to
   eliminate certain cases that pass Miller-Rabin but are not prime.
1. Run the conventional Baillie-PSW test: a strong base-2 Miller-Rabin test
   followed by a strong Lucas test using Selfridge's method-A parameters.
1. Run log2(bitlength) + 4 additional Miller-Rabin tests with random bases.

Safe primes require (n-1)/2 also be prime.

## Prime Checking

You can use this crate to check numbers for primality.

```rust
use glass_pumpkin::prime;
use glass_pumpkin::safe_prime;
use num_bigint::BigUint;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_prime = prime::check(&BigUint::new([5].to_vec()))?;
    let is_safe_prime = safe_prime::check(&BigUint::new([7].to_vec()))?;
    let _results = (is_prime, is_safe_prime);

    Ok(())
}
```

Deterministic-base probable-prime checking with the Baillie-PSW method is
available through the `strong_check` methods in the `prime` and `safe_prime`
modules. Prime generation uses the same test and follows it with the additional
random Miller-Rabin rounds described above.

This crate is part of the [LF Decentralized Trust Labs Agora
project](https://github.com/LF-Decentralized-Trust-labs/agora).

[//]: # (badges)

[build-image]: https://github.com/LF-Decentralized-Trust-labs/agora-glass_pumpkin/actions/workflows/glass_pumpkin.yml/badge.svg?branch=main
[build-link]: https://github.com/LF-Decentralized-Trust-labs/agora-glass_pumpkin/actions/workflows/glass_pumpkin.yml
[crate-image]: https://img.shields.io/crates/v/glass_pumpkin.svg
[crate-link]: https://crates.io/crates/glass_pumpkin
[docs-image]: https://docs.rs/glass_pumpkin/badge.svg
[docs-link]: https://docs.rs/glass_pumpkin/
[license-badge]: https://img.shields.io/badge/license-MIT/Apache2.0-blue.svg
