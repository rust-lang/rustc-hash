# rustc-hash

[![crates.io](https://img.shields.io/crates/v/rustc-hash.svg)](https://crates.io/crates/rustc-hash)
[![Documentation](https://docs.rs/rustc-hash/badge.svg)](https://docs.rs/rustc-hash)

A speedy, non-cryptographic hashing algorithm used by `rustc` and Firefox.
The [hash map in `std`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) uses SipHash by default, which provides resistance against DOS attacks.
These attacks aren't as much of a concern in the compiler so we prefer to use the quicker, non-cryptographic Fx algorithm.

The Fx algorithm is a unique one used by Firefox. It is fast because it can hash eight bytes at a time.
Within `rustc`, it consistently outperforms every other tested algorithm (such as FNV).
The collision rate is similar or slightly worse than other low-quality hash functions.

## Usage

This crate provides `FxHashMap` and `FxHashSet` as collections.
They are simply type aliases for their `std::collection` counterparts using the Fx hasher.

```rust
use rustc_hash::FxHashMap;

let mut map: FxHashMap<u32, u32> = FxHashMap::default();
map.insert(22, 44);
```

### `no_std`

The `std` feature is on by default to enable collections.
It can be turned off in `Cargo.toml` like so:

```toml
rustc-hash = { version = "1.1", default-features = false }
```
