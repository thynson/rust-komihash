use std::num::Wrapping;

#[inline(always)]
#[cold]
fn komihash_cold_path() {

}

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

#[inline]
fn as_array<const N: usize >(slice: &[u8]) -> &[u8; N] {
    debug_assert!(slice.len() >= N);
    unsafe { &*(slice.as_ptr() as *const [_; N]) }
}

#[inline(always)]
pub fn multiply128(m1: Wrapping<u64>, m2: Wrapping<u64>) -> (Wrapping<u64>, Wrapping<u64>) {
    let u128: u128 = (m1.0 as u128) * (m2.0 as u128);
    (Wrapping(u128 as u64), Wrapping((u128 >> 64) as u64))
}

#[inline(always)]
pub fn read_word(buffer: &[u8]) -> Wrapping<u64> {
    // let mut tmp_buffer = [0u8; 8];
    // tmp_buffer.copy_from_slice(&buffer[0..8]);
    return Wrapping(u64::from_le_bytes(*as_array::<8>(buffer)));
}

#[inline(always)]
pub fn read_partial_word(mut buff: &[u8]) -> Wrapping<u64> {
    let mut ret = 0u64;
    let mut shift: u32 = 0;
    if buff.len() >= 4 {
        ret |= u32::from_le_bytes(*as_array::<4>(buff)) as u64;
        buff = &buff[4..];
        shift += 32;
    }

    if buff.len() >= 2 {
        ret |= (u16::from_le_bytes(*as_array::<2>(buff)) as u64) << shift;
        buff = &buff[2..];
        shift += 16;
    }

    if buff.len() > 0 {
        ret |= (buff[0] as u64) << shift;
    }

    Wrapping(ret)
}

