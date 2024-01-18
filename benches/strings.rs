#![feature(test)]

extern crate fnv;
extern crate rand;
extern crate rustc_hash;

extern crate test;
use test::{black_box, Bencher};

use std::collections::HashSet;
use std::hash::BuildHasher;

use fnv::FnvHashSet;
use rand::{distributions::Alphanumeric, rngs::StdRng, Rng, SeedableRng};
use rustc_hash::FxHashSet;

fn strings<const N: usize, const M: usize, H>() -> HashSet<String, H>
where
    H: BuildHasher + Default,
{
    let mut strings = HashSet::default();

    let rng = &mut StdRng::seed_from_u64(42);

    while strings.len() < M {
        let length = rng.gen_range(0..N);

        let string: String = rng
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        strings.insert(string);
    }

    strings
}

fn find_strings<H>(strings: &HashSet<String, H>) -> bool
where
    H: BuildHasher,
{
    let haystack = black_box(strings);
    let needles = black_box(strings);

    needles.iter().all(|needle| haystack.contains(needle))
}

macro_rules! compare_fx_fnv {
    ($name:ident, $string_length:expr, $table_size:expr) => {
        mod $name {
            use super::*;

            #[bench]
            fn fx(bencher: &mut Bencher) {
                let strings: FxHashSet<String> = strings::<$string_length, $table_size, _>();
                bencher.iter(|| find_strings(&strings));
            }

            #[bench]
            fn fnv(bencher: &mut Bencher) {
                let strings: FnvHashSet<String> = strings::<$string_length, $table_size, _>();
                bencher.iter(|| find_strings(&strings));
            }
        }
    };
}

compare_fx_fnv!(few_tiny, 3, 1_000);
compare_fx_fnv!(few_small, 7, 1_000);
compare_fx_fnv!(few_medium, 15, 1_000);
compare_fx_fnv!(few_large, 47, 1_000);

compare_fx_fnv!(some_small, 7, 10_000);
compare_fx_fnv!(some_medium, 15, 10_000);
compare_fx_fnv!(some_large, 47, 10_000);

compare_fx_fnv!(many_small, 7, 100_000);
compare_fx_fnv!(many_medium, 15, 100_000);
compare_fx_fnv!(many_large, 47, 100_000);
