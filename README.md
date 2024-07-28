# rustc-hash

[![crates.io](https://img.shields.io/crates/v/rustc-hash.svg)](https://crates.io/crates/rustc-hash)
[![Documentation](https://docs.rs/rustc-hash/badge.svg)](https://docs.rs/rustc-hash)

A speedy, non-cryptographic hashing algorithm used by `rustc`.
The [hash map in `std`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) uses SipHash by default, which provides resistance against DOS attacks.
These attacks aren't a concern in the compiler so we prefer to use a quicker,
non-cryptographic hash algorithm.

The original hash algorithm provided by this crate was one taken from Firefox,
hence the hasher it provides is called FxHasher. This name is kept for backwards
compatibility, but the underlying hash has since been replaced. The current
design for the hasher is a polynomial hash finished with a single bit rotation,
together with a wyhash-inspired compression function for strings/slices, both
designed by Orson Peters.

For `rustc` we have tried many different hashing algorithms. Hashing speed is
critical, especially for single integers. Spending more CPU cycles on a higher
quality hash does not reduce hash collisions enough to make the compiler faster
on real-world benchmarks.

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
rustc-hash = { version = "2.0", default-features = false }
```

## Benchmarks

The benchmarks are run with the following command:

- `rustc-hash::FxHashMap`

```console
> cargo +nightly bench

test fx_benchmarks::bench_hashmap_create                 ... bench:           2.08 ns/iter (+/- 0.04)
test fx_benchmarks::bench_hashmap_insert                 ... bench:         175.02 ns/iter (+/- 5.89)
test fx_benchmarks::bench_hashmap_insert_large_data      ... bench:  16,121,283.40 ns/iter (+/- 2,276,733.63)
test fx_benchmarks::bench_hashmap_iter                   ... bench:           2.05 ns/iter (+/- 0.07)
test fx_benchmarks::bench_hashmap_iter_large_data        ... bench:     173,197.71 ns/iter (+/- 3,914.19)
test fx_benchmarks::bench_hashmap_lookup                 ... bench:           2.44 ns/iter (+/- 0.11)
test fx_benchmarks::bench_hashmap_lookup_large_data      ... bench:     444,745.90 ns/iter (+/- 91,353.67)
test fx_benchmarks::bench_hashmap_reinsert               ... bench:          67.53 ns/iter (+/- 5.21)
test fx_benchmarks::bench_hashmap_reinsert_large_data    ... bench:  12,455,191.70 ns/iter (+/- 9,219,748.08)
test fx_benchmarks::bench_hashmap_remove                 ... bench:           4.95 ns/iter (+/- 0.17)
test fx_benchmarks::bench_hashmap_remove_large_data      ... bench:     561,822.95 ns/iter (+/- 14,508.17)
test fx_benchmarks::bench_hashmap_with_mutex             ... bench:          77.26 ns/iter (+/- 15.62)
test fx_benchmarks::bench_hashmap_with_mutex_large_data  ... bench:  13,674,849.90 ns/iter (+/- 9,925,902.12)
test fx_benchmarks::bench_hashmap_with_rwlock            ... bench:          73.16 ns/iter (+/- 2.48)
test fx_benchmarks::bench_hashmap_with_rwlock_large_data ... bench:  12,159,066.70 ns/iter (+/- 9,738,272.64)

test result: ok. 0 passed; 0 failed; 0 ignored; 15 measured; 0 filtered out; finished in 37.61s
```

- `std::collections::HashMap`

```console
> cargo +nightly bench --features=std-bench

...
test std_benchmarks::bench_hashmap_create                 ... bench:           9.20 ns/iter (+/- 0.14)
test std_benchmarks::bench_hashmap_insert                 ... bench:         195.85 ns/iter (+/- 6.37)
test std_benchmarks::bench_hashmap_insert_large_data      ... bench:  21,693,487.50 ns/iter (+/- 3,609,774.56)
test std_benchmarks::bench_hashmap_iter                   ... bench:           2.08 ns/iter (+/- 0.05)
test std_benchmarks::bench_hashmap_iter_large_data        ... bench:     226,075.13 ns/iter (+/- 9,295.55)
test std_benchmarks::bench_hashmap_lookup                 ... bench:          16.44 ns/iter (+/- 0.37)
test std_benchmarks::bench_hashmap_lookup_large_data      ... bench:   2,456,185.40 ns/iter (+/- 109,599.74)
test std_benchmarks::bench_hashmap_reinsert               ... bench:          84.26 ns/iter (+/- 2.26)
test std_benchmarks::bench_hashmap_reinsert_large_data    ... bench:  10,378,541.70 ns/iter (+/- 5,037,469.76)
test std_benchmarks::bench_hashmap_remove                 ... bench:          17.83 ns/iter (+/- 0.13)
test std_benchmarks::bench_hashmap_remove_large_data      ... bench:   3,100,606.20 ns/iter (+/- 28,839.83)
test std_benchmarks::bench_hashmap_with_mutex             ... bench:          96.03 ns/iter (+/- 4.22)
test std_benchmarks::bench_hashmap_with_mutex_large_data  ... bench:  11,495,400.00 ns/iter (+/- 4,018,753.31)
test std_benchmarks::bench_hashmap_with_rwlock            ... bench:          89.40 ns/iter (+/- 3.91)
test std_benchmarks::bench_hashmap_with_rwlock_large_data ... bench:  12,300,512.40 ns/iter (+/- 11,790,539.19)

test result: ok. 0 passed; 0 failed; 0 ignored; 15 measured; 0 filtered out; finished in 42.77s
```
