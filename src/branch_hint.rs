//!
//! Compiler hint for branch prediction.
//!
//! See: https://users.rust-lang.org/t/compiler-hint-for-unlikely-likely-for-if-branches/62102/4
//!

#[inline(always)]
#[cold]
fn komihash_cold_path() {}

#[inline(always)]
pub fn komihash_likely(b: bool) -> bool {
    if !b {
        komihash_cold_path();
    }

    b
}

#[inline(always)]
pub fn komihash_unlikely(b: bool) -> bool {
    if b {
        komihash_cold_path();
    }
    b
}
