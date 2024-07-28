#![cfg(feature = "nightly")]
#![feature(test)]

extern crate test;

use std::sync::{Arc, Mutex, RwLock};
use test::{black_box, Bencher};

#[cfg(not(feature = "std_bench"))]
use rustc_hash::FxHashMap;

#[cfg(feature = "std_bench")]
use std::collections::HashMap;

const LARGE_ITEM_SIZE: usize = 100000;

macro_rules! benchmarks {
    ($map_type:ty,$map_creater:expr) => {
        // Benchmark for creating a FxHashMap.
        #[bench]
        fn bench_hashmap_create(b: &mut Bencher) {
            b.iter(|| {
                let map: $map_type = $map_creater();
                black_box(map)
            });
        }

        // Benchmark for inserting items into a FxHashMap.
        #[bench]
        fn bench_hashmap_insert(b: &mut Bencher) {
            b.iter(|| {
                let mut map = $map_creater();
                map.insert(1, 1.to_string());
                black_box(map)
            });
        }

        #[bench]
        fn bench_hashmap_insert_large_data(b: &mut Bencher) {
            b.iter(|| {
                let mut map = $map_creater();
                for i in 0..LARGE_ITEM_SIZE {
                    map.insert(i, i.to_string());
                }
                black_box(map)
            });
        }

        // Benchmark for looking up items in a FxHashMap.
        #[bench]
        fn bench_hashmap_lookup(b: &mut Bencher) {
            let mut map = $map_creater();
            map.insert(1, 1.to_string());
            b.iter(|| {
                black_box(map.get(&1));
            });
        }

        #[bench]
        fn bench_hashmap_lookup_large_data(b: &mut Bencher) {
            let mut map = $map_creater();
            for i in 0..LARGE_ITEM_SIZE {
                map.insert(i, i.to_string());
            }
            b.iter(|| {
                for i in 0..LARGE_ITEM_SIZE {
                    black_box(map.get(&i));
                }
            });
        }

        // Benchmark for removing items from a FxHashMap.
        #[bench]
        fn bench_hashmap_remove(b: &mut Bencher) {
            let mut map = $map_creater();
            map.insert(1, 1.to_string());
            b.iter(|| {
                black_box(map.remove(&1));
            });
        }

        #[bench]
        fn bench_hashmap_remove_large_data(b: &mut Bencher) {
            let mut map = $map_creater();
            for i in 0..LARGE_ITEM_SIZE {
                map.insert(i, i.to_string());
            }
            b.iter(|| {
                for i in 0..LARGE_ITEM_SIZE {
                    black_box(map.remove(&i));
                }
            });
        }

        // Benchmark for iterating over a FxHashMap.
        #[bench]
        fn bench_hashmap_iter(b: &mut Bencher) {
            let mut map = $map_creater();
            map.insert(1, 1.to_string());
            b.iter(|| {
                for (k, v) in &map {
                    black_box((k, v));
                }
            });
        }

        #[bench]
        fn bench_hashmap_iter_large_data(b: &mut Bencher) {
            let mut map = $map_creater();
            for i in 0..LARGE_ITEM_SIZE {
                map.insert(i, i.to_string());
            }
            b.iter(|| {
                for (k, v) in &map {
                    black_box((k, v));
                }
            });
        }

        // Benchmark for reinserting items into a FxHashMap.
        #[bench]
        fn bench_hashmap_reinsert(b: &mut Bencher) {
            let mut map = $map_creater();
            map.insert(1, 1.to_string());
            b.iter(|| {
                map.insert(1, 2.to_string());
            });
        }

        #[bench]
        fn bench_hashmap_reinsert_large_data(b: &mut Bencher) {
            let mut map = $map_creater();
            for i in 0..LARGE_ITEM_SIZE {
                map.insert(i, i.to_string());
            }
            b.iter(|| {
                for i in 0..LARGE_ITEM_SIZE {
                    map.insert(i, (i + 1).to_string());
                }
            });
        }

        // Benchmark for inserting items into a FxHashMap with a Mutex.
        #[bench]
        fn bench_hashmap_with_mutex(b: &mut Bencher) {
            let map = Arc::new(Mutex::new($map_creater()));
            b.iter(|| {
                let mut locked_map = map.lock().unwrap();
                locked_map.insert(1, 1.to_string());
            });
        }

        #[bench]
        fn bench_hashmap_with_mutex_large_data(b: &mut Bencher) {
            let map = Arc::new(Mutex::new($map_creater()));
            b.iter(|| {
                for i in 0..LARGE_ITEM_SIZE {
                    let mut locked_map = map.lock().unwrap();
                    locked_map.insert(i, i.to_string());
                }
            });
        }

        // Benchmark for inserting items into a FxHashMap with a RwLock.
        #[bench]
        fn bench_hashmap_with_rwlock(b: &mut Bencher) {
            let map = Arc::new(RwLock::new($map_creater()));
            b.iter(|| {
                let mut locked_map = map.write().unwrap();
                locked_map.insert(1, 1.to_string());
            });
        }

        #[bench]
        fn bench_hashmap_with_rwlock_large_data(b: &mut Bencher) {
            let map = Arc::new(RwLock::new($map_creater()));
            b.iter(|| {
                for i in 0..LARGE_ITEM_SIZE {
                    let mut locked_map = map.write().unwrap();
                    locked_map.insert(i, i.to_string());
                }
            });
        }
    };
}

#[cfg(not(feature = "std_bench"))]
mod fx_benchmarks {
    use super::*;
    benchmarks!(FxHashMap<i32, String>, || FxHashMap::default());
}

#[cfg(feature = "std_bench")]
mod std_benchmarks {
    use super::*;
    benchmarks!(HashMap<i32, String>,|| HashMap::new());
}
