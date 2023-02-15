// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Fast, non-cryptographic hash used by rustc and Firefox.
//!
//! # Example
//!
//! ```rust
//! # #[cfg(feature = "std")]
//! # fn main() {
//! use rustc_hash::FxHashMap;
//! let mut map: FxHashMap<u32, u32> = FxHashMap::default();
//! map.insert(22, 44);
//! # }
//! # #[cfg(not(feature = "std"))]
//! # fn main() { }
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use core::hash::BuildHasherDefault;
use core::{convert::TryInto, default::Default, hash::Hasher, mem::size_of, ops::BitXor};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

/// Type alias for a hashmap using the `fx` hash algorithm.
#[cfg(feature = "std")]
#[allow(rustc::default_hash_types)]
pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

/// Type alias for a hashmap using the `fx` hash algorithm.
#[cfg(feature = "std")]
#[allow(rustc::default_hash_types)]
pub type FxHashSet<V> = HashSet<V, BuildHasherDefault<FxHasher>>;


#[cfg(target_pointer_width = "32")]
const K: usize = 0x9e3779b9;
#[cfg(target_pointer_width = "64")]
const K: usize = 0x517cc1b727220a95;

#[inline]
fn merge(a: usize, b: usize) -> usize {
    a.rotate_left(5).bitxor(b).wrapping_mul(K)
}

/// A speedy hash algorithm for use within rustc. The hashmap in liballoc
/// by default uses SipHash which isn't quite as speedy as we want. In the
/// compiler we're not really worried about DOS attempts, so we use a fast
/// non-cryptographic hash.
///
/// This is the same as the algorithm used by Firefox -- which is a homespun
/// one not based on any widely-known algorithm -- though modified to produce
/// 64-bit hash values instead of 32-bit hash values. It consistently
/// out-performs an FNV-based hash within rustc itself -- the collision rate is
/// similar or slightly worse than FNV, but the speed of the hash function
/// itself is much higher because it works on up to 32 bytes at a time.
#[derive(Default)]
pub struct FxHasher {
    hash: usize,
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        const _: () = assert!(
            size_of::<usize>() <= 8,
            "usize > 64bit platforms not supported"
        );
        const ULEN: usize = size_of::<usize>();

        // use 4-wide hasher for any data big enough to split into multiple lanes
        if bytes.len() >= ULEN * 2 {
            let mut wide = FxHasher4Wide::default();
            wide.write(bytes);
            self.hash = merge(self.hash, wide.finish_usize());
            return;
        }

        // note: avoids repeated *self dereference
        let mut hash = self.hash;
        if bytes.len() >= ULEN {
            hash = merge(
                hash,
                usize::from_ne_bytes(bytes[..ULEN].try_into().unwrap()),
            );
            bytes = &bytes[ULEN..];
        }
        if ULEN > 4 && bytes.len() >= 4 {
            hash = merge(
                hash,
                u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as _,
            );
            bytes = &bytes[4..];
        }
        if ULEN > 2 && bytes.len() >= 2 {
            hash = merge(
                hash,
                u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as _,
            );
            bytes = &bytes[2..];
        }
        if ULEN > 1 && !bytes.is_empty() {
            hash = merge(hash, bytes[0] as _);
        }

        self.hash = hash;
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash = merge(self.hash, i as _);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash = merge(self.hash, i as _);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash = merge(self.hash, i as _);
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.hash = merge(self.hash, i as _);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.hash = merge(self.hash, i);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash as _
    }
}

/// Hasher that stripes data into 4 fx-hash lanes.
#[derive(Default)]
struct FxHasher4Wide {
    hash: [usize; 4],
}

impl FxHasher4Wide {
    #[inline]
    fn finish_usize(&self) -> usize {
        let mut hash = merge(self.hash[0], self.hash[1]);
        hash = merge(hash, self.hash[2]);
        merge(hash, self.hash[3])
    }
}

impl Hasher for FxHasher4Wide {
    // Write impl that should auto-vectorize well.
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        const _: () = assert!(
            size_of::<usize>() <= 8,
            "usize > 64bit platforms not supported"
        );
        const ULEN: usize = size_of::<usize>();
        const ULEN2: usize = ULEN * 2;
        const ULEN4: usize = ULEN * 4;

        // note: avoids repeated *self dereference
        let mut hash = self.hash;
        while bytes.len() >= ULEN4 {
            let n = unsafe {
                core::mem::transmute::<[u8; ULEN4], [usize; 4]>(bytes[..ULEN4].try_into().unwrap())
            };
            hash[0] = merge(hash[0], n[0]);
            hash[1] = merge(hash[1], n[1]);
            hash[2] = merge(hash[2], n[2]);
            hash[3] = merge(hash[3], n[3]);
            bytes = &bytes[ULEN4..];
        }
        if bytes.len() >= ULEN2 {
            let n = unsafe {
                core::mem::transmute::<[u8; ULEN2], [usize; 2]>(bytes[..ULEN2].try_into().unwrap())
            };
            hash[0] = merge(hash[0], n[0]);
            hash[1] = merge(hash[1], n[1]);
            bytes = &bytes[ULEN2..];
        }
        if bytes.len() >= ULEN {
            hash[0] = merge(
                hash[0],
                usize::from_ne_bytes(bytes[..ULEN].try_into().unwrap()),
            );
            bytes = &bytes[ULEN..];
        }
        if ULEN > 4 && bytes.len() >= 4 {
            hash[0] = merge(
                hash[0],
                u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as _,
            );
            bytes = &bytes[4..];
        }
        if ULEN > 2 && bytes.len() >= 2 {
            hash[0] = merge(
                hash[0],
                u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as _,
            );
            bytes = &bytes[2..];
        }
        if ULEN > 1 && !bytes.is_empty() {
            hash[0] = merge(hash[0], bytes[0] as _);
        }
        self.hash = hash;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.finish_usize() as _
    }
}
