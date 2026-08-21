/*
 * This file includes a rust port of Komihash v5 implementation
 *
 * Copyright (c) 2023 LAN Xingcan
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use crate::utils::{komihash_likely, komihash_unlikely, multiply128, read_partial_word, read_word};
use std::hash::Hasher;
use std::num::Wrapping;
use std::ops::{Add, BitXor};

// Komihash v5.34+ changed the komirand recurrence, so v5 has its own
// implementation. See https://github.com/avaneev/komihash (komirand function).
pub struct Komirand {
    s1: Wrapping<u64>,
    s2: Wrapping<u64>,
}

impl Komirand {
    ///
    /// Creates a Komirand instance with the given seeds.
    ///
    /// Note: seeds can be initialized to any value (even 0), but the state
    /// should be warmed up (e.g. via [`Komirand::init`]) before use if the
    /// seeds are not high-quality random values.
    ///
    pub const fn new(s1: u64, s2: u64) -> Self {
        Self {
            s1: Wrapping(s1),
            s2: Wrapping(s2),
        }
    }

    ///
    /// Creates a Komirand instance with the a given seed, and warmup the state before returning.
    ///
    pub fn init(seed: u64) -> Self {
        let mut ret = Self::new(seed, seed);

        // warming up
        ret.next();
        ret.next();
        ret.next();
        ret.next();

        ret
    }

    ///
    /// Create a new Komirand instance with two seeds, and warmup the state before returning.
    ///
    pub fn init_with_extra_seed(s1: u64, s2: u64) -> Self {
        let mut ret = Self::new(s1, s2);

        // warming up
        ret.next();
        ret.next();
        ret.next();
        ret.next();

        ret
    }

    ///
    /// Advances the generator and returns the next 64-bit value.
    ///
    /// This implements the komirand recurrence used by komihash v5.34+:
    ///
    /// ```text
    /// (lo, hi) = s1 * s2 (128-bit)
    /// s1' = (lo ^ hi) + 0x5555555555555555
    /// s2' = s2 + hi + 0xAAAAAAAAAAAAAAAA
    /// ```
    ///
    /// Note: the output stream differs from [`crate::v4::Komirand`].
    ///
    pub fn next(&mut self) -> u64 {
        let (lo, hi) = multiply128(self.s1, self.s2);
        let mut s1 = lo.bitxor(hi);
        let mut s2 = self.s2.add(hi);
        s2 += Wrapping(0xaaaaaaaaaaaaaaaau64);
        s1 += Wrapping(0x5555555555555555u64);
        self.s1 = s1;
        self.s2 = s2;
        s1.0
    }

    ///
    /// Fills the buffer with pseudorandom bytes, in little-endian order.
    ///
    /// A fill of N bytes consumes exactly the generator outputs that `next()`
    /// would produce for those bytes; empty fills do not advance the state.
    ///
    pub fn fill_bytes(&mut self, mut buffer: &mut [u8]) {
        while buffer.len() >= 8 {
            buffer[..8].copy_from_slice(&self.next().to_le_bytes());
            buffer = &mut buffer[8..];
        }

        if buffer.is_empty() {
            return;
        }

        let mut last = self.next();

        if buffer.len() >= 4 {
            buffer[..4].copy_from_slice(&(last as u32).to_le_bytes()[..4]);
            last >>= 32;
            buffer = &mut buffer[4..];
        }

        if buffer.len() >= 2 {
            buffer[..2].copy_from_slice(&(last as u16).to_le_bytes()[..2]);
            last >>= 16;
            buffer = &mut buffer[2..];
        }

        if !buffer.is_empty() {
            buffer[0] = last as u8;
        }
    }
}

const KOMI_HASH_INTERNAL_BUFF_SIZE: usize = 64;

///
/// Streaming hash state of komihash.
///
/// Use this method when the input size is too large or unknown. For maximum performance,
/// make sure the buffer size aligned to 64 bytes to avoid contents being copied into the
/// internal buffer.
///
pub struct StreamedKomihash {
    seed1: Wrapping<u64>,
    seed2: Wrapping<u64>,
    seed3: Wrapping<u64>,
    seed4: Wrapping<u64>,
    seed5: Wrapping<u64>,
    seed6: Wrapping<u64>,
    seed7: Wrapping<u64>,
    seed8: Wrapping<u64>,
    buffer: [u8; KOMI_HASH_INTERNAL_BUFF_SIZE],
    bytes_count: usize,
}

///
/// One-shot komihash function
///
/// Recommended for use unless the input size is too large.
///
pub fn komihash(mut bytes: &[u8], seed: u64) -> u64 {
    let mut seed1 = Wrapping(0x243f6a8885a308d3 ^ (seed & 0x5555555555555555));
    let mut seed5 = Wrapping(0x452821e638d01377 ^ (seed & 0xaaaaaaaaaaaaaaaa));

    let (l, h) = multiply128(seed1, seed5);
    seed5 = seed5.add(h);
    seed1 = seed5.bitxor(l);

    if komihash_unlikely(bytes.is_empty()) {
        let (r3l, r3h) = multiply128(seed1, seed5);
        seed5 = seed5.add(r3h);
        seed1 = seed5.bitxor(r3l);

        let (r4l, r4h) = multiply128(seed1, seed5);
        seed5 = seed5.add(r4h);
        seed1 = seed5.bitxor(r4l);
        return seed1.0;
    }

    if komihash_likely(bytes.len() < 16) {
        return komihash_finish(bytes, seed1, seed5);
    }

    if komihash_likely(bytes.len() < 32) {
        // SAFETY: bytes.len() >= 16
        unsafe {
            let tmp1 = read_word(bytes);
            let tmp2 = read_word(&bytes[8..]);
            let (r1l, r1h) = multiply128(tmp1.bitxor(seed1), tmp2.bitxor(seed5));
            seed5 = seed5.add(r1h);
            seed1 = seed5.bitxor(r1l);
            bytes = &bytes[16..];
        }
        return komihash_finish(bytes, seed1, seed5);
    }

    if bytes.len() >= 64 {
        let mut seed2 = Wrapping(0x13198a2e03707344).bitxor(seed1);
        let mut seed3 = Wrapping(0xa4093822299f31d0).bitxor(seed1);
        let mut seed4 = Wrapping(0x082efa98ec4e6c89).bitxor(seed1);
        let mut seed6 = Wrapping(0xbe5466cf34e90c6c).bitxor(seed5);
        let mut seed7 = Wrapping(0xc0ac29b7c97c50dd).bitxor(seed5);
        let mut seed8 = Wrapping(0x3f84d5b5b5470917).bitxor(seed5);
        loop {
            // SAFETY: bytes.len() >= 64
            unsafe {
                let b0 = read_word(bytes);
                let b1 = read_word(&bytes[8..]);
                let b2 = read_word(&bytes[16..]);
                let b3 = read_word(&bytes[24..]);
                let b4 = read_word(&bytes[32..]);
                let b5 = read_word(&bytes[40..]);
                let b6 = read_word(&bytes[48..]);
                let b7 = read_word(&bytes[56..]);

                let (r1l, r1h) = multiply128(b0.bitxor(seed1), b4.bitxor(seed5));
                let (r2l, r2h) = multiply128(b1.bitxor(seed2), b5.bitxor(seed6));
                let (r3l, r3h) = multiply128(b2.bitxor(seed3), b6.bitxor(seed7));
                let (r4l, r4h) = multiply128(b3.bitxor(seed4), b7.bitxor(seed8));

                seed5 = seed5.add(r1h);
                seed2 = seed5.bitxor(r2l);
                seed6 = seed6.add(r2h);
                seed3 = seed6.bitxor(r3l);
                seed7 = seed7.add(r3h);
                seed4 = seed7.bitxor(r4l);
                seed8 = seed8.add(r4h);
                seed1 = seed8.bitxor(r1l);
            }

            bytes = &bytes[64..];
            if bytes.len() < 64 {
                break;
            }
        }

        seed5 = seed5.bitxor(seed6).bitxor(seed7).bitxor(seed8);
        seed1 = seed1.bitxor(seed2).bitxor(seed3).bitxor(seed4);
    }
    if bytes.len() >= 32 {
        // SAFETY: bytes.len() >= 32
        unsafe {
            let tmp1 = read_word(bytes);
            let tmp2 = read_word(&bytes[8..]);
            let tmp3 = read_word(&bytes[16..]);
            let tmp4 = read_word(&bytes[24..]);
            let (r1l, r1h) = multiply128(tmp1.bitxor(seed1), tmp2.bitxor(seed5));
            seed5 = seed5.add(r1h);
            seed1 = seed5.bitxor(r1l);

            let (r2l, r2h) = multiply128(tmp3.bitxor(seed1), tmp4.bitxor(seed5));
            seed5 = seed5.add(r2h);
            seed1 = seed5.bitxor(r2l);
        }

        bytes = &bytes[32..];
    }

    if bytes.len() >= 16 {
        // SAFETY: bytes.len() >= 16
        unsafe {
            let tmp1 = read_word(bytes);
            let tmp2 = read_word(&bytes[8..]);
            let (r1l, r1h) = multiply128(tmp1.bitxor(seed1), tmp2.bitxor(seed5));
            seed5 = seed5.add(r1h);
            seed1 = seed5.bitxor(r1l);
        }

        bytes = &bytes[16..];
    }

    komihash_finish(bytes, seed1, seed5)
}
#[inline]
fn komihash_finish(mut bytes: &[u8], mut seed1: Wrapping<u64>, mut seed5: Wrapping<u64>) -> u64 {
    let mut r2l = seed1;
    let mut r2h = seed5;

    if komihash_likely(bytes.len() >= 8) {
        // SAFETY: bytes.len() >= 8
        let b0 = unsafe { read_word(bytes) };
        let tmp = r2l.bitxor(b0);
        r2l = r2h;
        r2h = tmp;
        bytes = &bytes[8..];
    }

    if komihash_likely(!bytes.is_empty()) {
        let ml8 = bytes.len() << 3;
        let b0 = read_partial_word(bytes);
        let fb = Wrapping(1u64 << ml8);
        r2l = r2l.bitxor(fb | b0);
    } else {
        let fb = Wrapping(1u64);
        r2l = r2l.bitxor(fb);
    }

    let (r3l, r3h) = multiply128(r2l, r2h);
    seed5 = seed5.add(r3h);
    seed1 = seed5.bitxor(r3l);

    let (r4l, r4h) = multiply128(seed1, seed5);
    seed5 = seed5.add(r4h);
    seed1 = seed5.bitxor(r4l);
    seed1.0
}

impl StreamedKomihash {
    ///
    /// Create a streamed komihash instance with default seed
    ///
    pub fn new() -> Self {
        StreamedKomihash::new_with_seed(0)
    }

    ///
    /// Create a streamed komihash instance with specified seed
    ///
    pub fn new_with_seed(seed: u64) -> StreamedKomihash {
        let mut seed1 = Wrapping(0x243f6a8885a308d3 ^ (seed & 0x5555555555555555));
        let mut seed5 = Wrapping(0x452821e638d01377 ^ (seed & 0xaaaaaaaaaaaaaaaa));

        let (l, h) = multiply128(seed1, seed5);
        seed5 = seed5.add(h);
        seed1 = seed5.bitxor(l);
        let seed2 = Wrapping(0x13198a2e03707344).bitxor(seed1);
        let seed3 = Wrapping(0xa4093822299f31d0).bitxor(seed1);
        let seed4 = Wrapping(0x082efa98ec4e6c89).bitxor(seed1);
        let seed6 = Wrapping(0xbe5466cf34e90c6c).bitxor(seed5);
        let seed7 = Wrapping(0xc0ac29b7c97c50dd).bitxor(seed5);
        let seed8 = Wrapping(0x3f84d5b5b5470917).bitxor(seed5);

        StreamedKomihash {
            seed1,
            seed2,
            seed3,
            seed4,
            seed5,
            seed6,
            seed7,
            seed8,
            buffer: [0; KOMI_HASH_INTERNAL_BUFF_SIZE],
            bytes_count: 0,
        }
    }

    #[inline]
    fn process_buffer(&mut self) {
        // SAFETY: self.buffer.len() is exactly 64
        unsafe {
            let b0 = read_word(&self.buffer[0..]);
            let b1 = read_word(&self.buffer[8..]);
            let b2 = read_word(&self.buffer[16..]);
            let b3 = read_word(&self.buffer[24..]);
            let b4 = read_word(&self.buffer[32..]);
            let b5 = read_word(&self.buffer[40..]);
            let b6 = read_word(&self.buffer[48..]);
            let b7 = read_word(&self.buffer[56..]);
            self.process_state(b0, b1, b2, b3, b4, b5, b6, b7);
        }
    }

    #[inline]
    fn process_state(
        &mut self,
        b0: Wrapping<u64>,
        b1: Wrapping<u64>,
        b2: Wrapping<u64>,
        b3: Wrapping<u64>,
        b4: Wrapping<u64>,
        b5: Wrapping<u64>,
        b6: Wrapping<u64>,
        b7: Wrapping<u64>,
    ) {
        let (r1l, r1h) = multiply128(b0.bitxor(self.seed1), b4.bitxor(self.seed5));
        let (r2l, r2h) = multiply128(b1.bitxor(self.seed2), b5.bitxor(self.seed6));
        let (r3l, r3h) = multiply128(b2.bitxor(self.seed3), b6.bitxor(self.seed7));
        let (r4l, r4h) = multiply128(b3.bitxor(self.seed4), b7.bitxor(self.seed8));

        self.seed5 = self.seed5.add(r1h);
        self.seed6 = self.seed6.add(r2h);
        self.seed7 = self.seed7.add(r3h);
        self.seed8 = self.seed8.add(r4h);
        self.seed2 = self.seed5.bitxor(r2l);
        self.seed3 = self.seed6.bitxor(r3l);
        self.seed4 = self.seed7.bitxor(r4l);
        self.seed1 = self.seed8.bitxor(r1l);
    }

    pub fn finish(&self) -> u64 {
        let mut seed5 = self.seed5;
        let mut seed1 = self.seed1;
        if komihash_unlikely(self.bytes_count == 0) {
            let (r3l, r3h) = multiply128(seed1, seed5);
            seed5 = seed5.add(r3h);
            seed1 = seed5.bitxor(r3l);

            let (r4l, r4h) = multiply128(seed1, seed5);
            seed5 = seed5.add(r4h);
            seed1 = seed5.bitxor(r4l);
            return seed1.0;
        }

        if self.bytes_count >= 64 {
            seed5 = seed5
                .bitxor(self.seed6)
                .bitxor(self.seed7)
                .bitxor(self.seed8);
            seed1 = seed1
                .bitxor(self.seed2)
                .bitxor(self.seed3)
                .bitxor(self.seed4);
        }
        let mut remaining = &self.buffer[0..(self.bytes_count & 0x3F)];

        if remaining.len() >= 32 {
            // SAFETY: bytes.len() >= 32
            unsafe {
                let b0 = read_word(&remaining[0..]);
                let b1 = read_word(&remaining[8..]);
                let b2 = read_word(&remaining[16..]);
                let b3 = read_word(&remaining[24..]);

                let tmp1 = seed1.bitxor(b0);
                let tmp2 = seed5.bitxor(b1);
                let (r1l, r1h) = multiply128(tmp1, tmp2);
                seed5 = seed5.add(r1h);
                seed1 = seed5.bitxor(r1l);

                let tmp3 = seed1.bitxor(b2);
                let tmp4 = seed5.bitxor(b3);

                let (r2l, r2h) = multiply128(tmp3, tmp4);
                seed5 = seed5.add(r2h);
                seed1 = seed5.bitxor(r2l);
            }

            remaining = &remaining[32..];
        }

        if remaining.len() >= 16 {
            // SAFETY: reamining.len() >= 16
            unsafe {
                let b0 = read_word(remaining);
                let b1 = read_word(&remaining[8..]);

                let tmp1 = seed1.bitxor(b0);
                let tmp2 = seed5.bitxor(b1);
                let (r1l, r1h) = multiply128(tmp1, tmp2);
                seed5 = seed5.add(r1h);
                seed1 = seed5.bitxor(r1l);
            }

            remaining = &remaining[16..];
        }
        komihash_finish(remaining, seed1, seed5)
    }

    pub fn write(&mut self, mut bytes: &[u8]) {
        let mut offset = self.bytes_count & 0x3F;
        self.bytes_count += bytes.len();
        let x = 64 - offset;
        if bytes.len() >= x {
            if offset != 0 {
                self.buffer[offset..64].copy_from_slice(&bytes[0..x]);
                bytes = &bytes[x..];
                self.process_buffer();
                offset = 0;
            }

            while bytes.len() >= 64 {
                // SAFETY: bytes.len() >= 64
                unsafe {
                    let b0 = read_word(&bytes[0..]);
                    let b1 = read_word(&bytes[8..]);
                    let b2 = read_word(&bytes[16..]);
                    let b3 = read_word(&bytes[24..]);
                    let b4 = read_word(&bytes[32..]);
                    let b5 = read_word(&bytes[40..]);
                    let b6 = read_word(&bytes[48..]);
                    let b7 = read_word(&bytes[56..]);
                    self.process_state(b0, b1, b2, b3, b4, b5, b6, b7);
                }
                bytes = &bytes[64..];
            }
        }

        self.buffer[offset..offset + bytes.len()].copy_from_slice(bytes);
    }
}

impl Default for StreamedKomihash {
    fn default() -> Self {
        StreamedKomihash::new()
    }
}

pub struct KomiHasher {
    seed: u64,
}

impl KomiHasher {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
}

impl Default for KomiHasher {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Hasher for KomiHasher {
    fn finish(&self) -> u64 {
        self.seed
    }

    fn write(&mut self, bytes: &[u8]) {
        // As the Hasher trait explicitly states that different call to write() should not be
        // treated as if the bytes were concatenated, this is a valid implementation.
        // Also the author of komihash recommended to implement discrete incremental hashing
        // by using the previous hash value as the seed for the next call
        // See: https://github.com/avaneev/komihash#discrete-incremental-hashing
        self.seed = komihash(bytes, self.seed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_vector_v5::TestSpec;
    use crate::tests::test_vector_v5::{
        komihash_test_vector_extended, komihash_test_vector_official,
    };

    #[test]
    fn test_with_official_test_vector() {
        for TestSpec {
            seed,
            input,
            output,
        } in komihash_test_vector_official()
        {
            assert_eq!(
                komihash(&input, seed),
                output,
                "content: {:?}, with seed {:?}",
                input,
                seed
            );
            let mut hasher = StreamedKomihash::new_with_seed(seed);
            hasher.write(&input);
            assert_eq!(
                hasher.finish(),
                output,
                "content: {:?}, with seed {:?}",
                input,
                seed
            );

            for chunk_size in 1..63 {
                let mut hasher = StreamedKomihash::new_with_seed(seed);
                for chunk in input.chunks(chunk_size) {
                    hasher.write(chunk);
                }
                assert_eq!(
                    hasher.finish(),
                    output,
                    "content: {:?}, chunked by: {:?}, with seed {:?}",
                    input,
                    chunk_size,
                    seed
                );
            }
        }
    }

    #[test]
    fn test_with_extended_test_vector() {
        for TestSpec {
            seed,
            input,
            output,
        } in komihash_test_vector_extended()
        {
            assert_eq!(
                komihash(&input, seed),
                output,
                "content: {:?}, with seed {:?}",
                input,
                seed
            );
            let mut hasher = StreamedKomihash::new_with_seed(seed);
            hasher.write(&input);
            assert_eq!(
                hasher.finish(),
                output,
                "content: {:?}, with seed {:?}",
                input,
                seed
            );

            for chunk_size in 1..63 {
                let mut hasher = StreamedKomihash::new_with_seed(seed);
                for chunk in input.chunks(chunk_size) {
                    hasher.write(chunk);
                }
                assert_eq!(
                    hasher.finish(),
                    output,
                    "content: {:?}, chunked by: {:?}, with seed {:?}",
                    input,
                    chunk_size,
                    seed
                );
            }
        }
    }

    #[test]
    fn test_komirand_v5_reference_vectors() {
        // Golden values generated from the komihash v5.34+ reference
        // recurrence (see komirand() in upstream komihash.h).
        let mut rand = Komirand::init(0x1234567890abcdef);
        assert_eq!(rand.next(), 0xb1b3fa0ce9e8fc12);
        assert_eq!(rand.next(), 0x873b3a41c1b9ada4);
        assert_eq!(rand.next(), 0x90ade4d9d4828c88);
        assert_eq!(rand.next(), 0x4887cba64ef55ec4);

        // Zero seed self-starts after warmup.
        assert_eq!(Komirand::init(0).next(), 0x1c42877a440ae8ee);
    }

    #[test]
    fn test_komirand_v5_fill_bytes_stream_continuity() {
        let mut one = Komirand::new(42, 42);
        let mut buf16 = [0u8; 16];
        one.fill_bytes(&mut buf16);

        let mut two = Komirand::new(42, 42);
        let mut buf8 = [0u8; 8];
        two.fill_bytes(&mut buf8);
        let first8 = buf8;
        two.fill_bytes(&mut buf8);

        assert_eq!(&buf16[..8], &first8[..]);
        assert_eq!(&buf16[8..], &buf8[..]);

        let mut with_empty = Komirand::new(42, 42);
        with_empty.fill_bytes(&mut []);
        let out = with_empty.next();
        let mut plain = Komirand::new(42, 42);
        assert_eq!(out, plain.next());
    }
}
