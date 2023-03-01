/*
 * Copyright (c) 2022 LAN Xingcan
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

/*
 * This file includes a rust port of Komihash implementation derives from hash4j
 * at https://github.com/dynatrace-oss/hash4j

 * Copyright 2022 Dynatrace LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod branch_hint;

use std::hash::Hasher;
use std::num::Wrapping;
use std::ops::{Add, BitXor};
use crate::branch_hint::{komihash_likely, komihash_unlikely};

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
    last_word: Wrapping<u64>,
    bytes_count: usize,
}

#[inline]
fn as_array<const N: usize >(slice: &[u8]) -> &[u8; N] {
    debug_assert!(slice.len() >= N);
    unsafe { &*(slice.as_ptr() as *const [_; N]) }
}

#[inline(always)]
fn multiply128(m1: Wrapping<u64>, m2: Wrapping<u64>) -> (Wrapping<u64>, Wrapping<u64>) {
    let u128: u128 = (m1.0 as u128) * (m2.0 as u128);
    (Wrapping(u128 as u64), Wrapping((u128 >> 64) as u64))
}

#[inline(always)]
fn read_word(buffer: &[u8]) -> Wrapping<u64> {
    // let mut tmp_buffer = [0u8; 8];
    // tmp_buffer.copy_from_slice(&buffer[0..8]);
    return Wrapping(u64::from_le_bytes(*as_array::<8>(buffer)));
}

#[inline(always)]
fn read_partial_word(mut buff: &[u8]) -> Wrapping<u64> {
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

#[inline]
fn komihash_finish(mut bytes: &[u8],
                   mut seed1: Wrapping<u64>,
                   mut seed5: Wrapping<u64>,
                   mut last_word: Wrapping<u64>) -> u64 {
    let mut r2l = seed1;
    let mut r2h = seed5;

    if komihash_likely(bytes.len() >= 8) {
        let b0 = read_word(bytes);
        last_word = b0;
        let tmp = r2l.bitxor(b0);
        r2l = r2h;
        r2h = tmp;
        bytes = &bytes[8..];
    }

    if komihash_likely(bytes.len() > 0) {
        let ml8 = bytes.len() << 3;
        let b0 = read_partial_word(bytes);
        let fb = Wrapping(1u64 << (b0.0 >> (ml8 - 1)) << ml8);
        r2l = r2l.bitxor(fb | b0);
    } else {
        let fb = Wrapping(1u64 << (last_word.0 >> 63));
        r2l = r2l.bitxor(fb);
    }

    let (r3l, r3h) = multiply128(r2l, r2h);
    seed5 = seed5.add(r3h);
    seed1 = seed5.bitxor(r3l);

    let (r4l, r4h) = multiply128(seed1, seed5);
    seed5 = seed5.add(r4h);
    seed1 = seed5.bitxor(r4l);
    return seed1.0;
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

    if komihash_unlikely(bytes.len() == 0) {
        let (r3l, r3h) = multiply128(seed1, seed5);
        seed5 = seed5.add(r3h);
        seed1 = seed5.bitxor(r3l);

        let (r4l, r4h) = multiply128(seed1, seed5);
        seed5 = seed5.add(r4h);
        seed1 = seed5.bitxor(r4l);
        return seed1.0;
    }

    let mut last_word = Wrapping(0);

    if komihash_likely(bytes.len() < 16) {
        return komihash_finish(bytes, seed1, seed5, last_word);
    }

    if komihash_likely(bytes.len() < 32) {
        let tmp1 = read_word(bytes);
        let tmp2 = read_word(&bytes[8..]);
        last_word = tmp2;
        let (r1l, r1h) = multiply128(tmp1.bitxor(seed1), tmp2.bitxor(seed5));
        seed5 = seed5.add(r1h);
        seed1 = seed5.bitxor(r1l);
        bytes = &bytes[16..];
        return komihash_finish(bytes, seed1, seed5, last_word);
    }

    if bytes.len() >= 64 {
        let mut seed2 = Wrapping(0x13198a2e03707344).bitxor(seed1);
        let mut seed3 = Wrapping(0xa4093822299f31d0).bitxor(seed1);
        let mut seed4 = Wrapping(0x082efa98ec4e6c89).bitxor(seed1);
        let mut seed6 = Wrapping(0xbe5466cf34e90c6c).bitxor(seed5);
        let mut seed7 = Wrapping(0xc0ac29b7c97c50dd).bitxor(seed5);
        let mut seed8 = Wrapping(0x3f84d5b5b5470917).bitxor(seed5);
        loop {
            let b0 = read_word(bytes);
            let b1 = read_word(&bytes[8..]);
            let b2 = read_word(&bytes[16..]);
            let b3 = read_word(&bytes[24..]);
            let b4 = read_word(&bytes[32..]);
            let b5 = read_word(&bytes[40..]);
            let b6 = read_word(&bytes[48..]);
            let b7 = read_word(&bytes[56..]);
            last_word = b7;

            let (r1l, r1h) = multiply128(b0.bitxor(seed1), b1.bitxor(seed5));
            let (r2l, r2h) = multiply128(b2.bitxor(seed2), b3.bitxor(seed6));
            let (r3l, r3h) = multiply128(b4.bitxor(seed3), b5.bitxor(seed7));
            let (r4l, r4h) = multiply128(b6.bitxor(seed4), b7.bitxor(seed8));

            bytes = &bytes[64..];

            seed5 = seed5.add(r1h);
            seed2 = seed5.bitxor(r2l);
            seed6 = seed6.add(r2h);
            seed3 = seed6.bitxor(r3l);
            seed7 = seed7.add(r3h);
            seed4 = seed7.bitxor(r4l);
            seed8 = seed8.add(r4h);
            seed1 = seed8.bitxor(r1l);
            if bytes.len() < 64 {
                break;
            }
        }

        seed5 = seed5.bitxor(seed6).bitxor(seed7).bitxor(seed8);
        seed1 = seed1.bitxor(seed2).bitxor(seed3).bitxor(seed4);
    }
    if bytes.len() > 31 {
        let tmp1 = read_word(bytes);
        let tmp2 = read_word(&bytes[8..]);
        let tmp3 = read_word(&bytes[16..]);
        let tmp4 = read_word(&bytes[24..]);
        let (r1l, r1h) = multiply128(tmp1.bitxor(seed1), tmp2.bitxor(seed5));
        seed5 = seed5.add(r1h);
        seed1 = seed5.bitxor(r1l);

        last_word = tmp4;

        let (r2l, r2h) = multiply128(tmp3.bitxor(seed1), tmp4.bitxor(seed5));
        seed5 = seed5.add(r2h);
        seed1 = seed5.bitxor(r2l);

        bytes = &bytes[32..];
    }

    if bytes.len() >= 16 {
        let tmp1 = read_word(bytes);
        let tmp2 = read_word(&bytes[8..]);
        last_word = tmp2;
        let (r1l, r1h) = multiply128(tmp1.bitxor(seed1), tmp2.bitxor(seed5));
        seed5 = seed5.add(r1h);
        seed1 = seed5.bitxor(r1l);

        bytes = &bytes[16..];
    }

    return komihash_finish(bytes, seed1, seed5, last_word);
}

impl StreamedKomihash {
    ///
    /// Create a streamed komihash instance with default seed
    ///
    pub fn new() -> StreamedKomihash {
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
            last_word: Wrapping(0),
            buffer: [0; KOMI_HASH_INTERNAL_BUFF_SIZE],
            bytes_count: 0,
        }
    }

    #[inline]
    fn process_buffer(&mut self) {
        let b0 = read_word(&self.buffer[0..]);
        let b1 = read_word(&self.buffer[8..]);
        let b2 = read_word(&self.buffer[16..]);
        let b3 = read_word(&self.buffer[24..]);
        let b4 = read_word(&self.buffer[32..]);
        let b5 = read_word(&self.buffer[40..]);
        let b6 = read_word(&self.buffer[48..]);
        let b7 = read_word(&self.buffer[56..]);
        self.last_word = b7;
        self.process_state(b0, b1, b2, b3, b4, b5, b6, b7);
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
        let (r1l, r1h) = multiply128(b0.bitxor(self.seed1), b1.bitxor(self.seed5));
        let (r2l, r2h) = multiply128(b2.bitxor(self.seed2), b3.bitxor(self.seed6));
        let (r3l, r3h) = multiply128(b4.bitxor(self.seed3), b5.bitxor(self.seed7));
        let (r4l, r4h) = multiply128(b6.bitxor(self.seed4), b7.bitxor(self.seed8));

        self.seed5 = self.seed5.add(r1h);
        self.seed6 = self.seed6.add(r2h);
        self.seed7 = self.seed7.add(r3h);
        self.seed8 = self.seed8.add(r4h);
        self.seed2 = self.seed5.bitxor(r2l);
        self.seed3 = self.seed6.bitxor(r3l);
        self.seed4 = self.seed7.bitxor(r4l);
        self.seed1 = self.seed8.bitxor(r1l);
    }
}

impl Default for StreamedKomihash {
    fn default() -> Self {
        StreamedKomihash::new()
    }
}

impl Hasher for StreamedKomihash {
    fn finish(&self) -> u64 {
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
            seed5 = seed5.bitxor(self.seed6).bitxor(self.seed7).bitxor(self.seed8);
            seed1 = seed1.bitxor(self.seed2).bitxor(self.seed3).bitxor(self.seed4);
        }
        let mut remaining = &self.buffer[0..(self.bytes_count & 0x3F)];
        let mut last_word = self.last_word;

        if remaining.len() > 31 {
            let b0 = read_word(&remaining[0..]);
            let b1 = read_word(&remaining[8..]);
            let b2 = read_word(&remaining[16..]);
            let b3 = read_word(&remaining[24..]);
            last_word = b3;

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

            remaining = &remaining[32..];
        }

        if remaining.len() > 15 {
            let b0 = read_word(remaining);
            let b1 = read_word(&remaining[8..]);
            last_word = b1;

            let tmp1 = seed1.bitxor(b0);
            let tmp2 = seed5.bitxor(b1);
            let (r1l, r1h) = multiply128(tmp1, tmp2);
            seed5 = seed5.add(r1h);
            seed1 = seed5.bitxor(r1l);

            remaining = &remaining[16..];
        }
        return komihash_finish(remaining, seed1, seed5, last_word);
    }

    fn write(&mut self, mut bytes: &[u8]) {
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
                let b0 = read_word(&bytes[0..]);
                let b1 = read_word(&bytes[8..]);
                let b2 = read_word(&bytes[16..]);
                let b3 = read_word(&bytes[24..]);
                let b4 = read_word(&bytes[32..]);
                let b5 = read_word(&bytes[40..]);
                let b6 = read_word(&bytes[48..]);
                let b7 = read_word(&bytes[56..]);
                self.last_word = b7;
                self.process_state(b0, b1, b2, b3, b4, b5, b6, b7);
                bytes = &bytes[64..];
            }
        }

        self.buffer[offset..offset + bytes.len()].copy_from_slice(bytes);
    }
}

pub struct Komirand {
    s1: Wrapping<u64>,
    s2: Wrapping<u64>,
}

impl Komirand {
    ///
    /// Creates a Komirand instance with the given seeds.
    ///
    pub const fn new(s1: u64, s2: u64) -> Self {
        Self { s1: Wrapping(s1), s2: Wrapping(s2) }
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

    pub fn next(&mut self) -> u64 {
        let (r1l, r1h) = multiply128(self.s1, self.s2);
        self.s1 = r1l;
        self.s2 += r1h.add(Wrapping(0xaaaaaaaaaaaaaaaau64));
        self.s1 ^= self.s2;
        self.s1.0
    }

    pub fn fill_bytes(&mut self, mut buffer: &mut [u8]) {
        while buffer.len() >= 8 {
            buffer[..8].copy_from_slice(&self.next().to_le_bytes());
            buffer = &mut buffer[8..];
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

        if buffer.len() > 0 {
            buffer[0] = last as u8;
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cmp::min;
    use std::hash::Hasher;

    use crate::{komihash, Komirand, read_word, StreamedKomihash};

    mod test_vector;

    use test_vector::test_vector;
    use crate::tests::test_vector::komi_rand_test_vector;


    #[test]
    fn hash_test() {
        for (hash0, hash1, seed, content) in test_vector() {
            if hash0 != 0xbb0f7d611b381083u64 {
                continue
            }
            assert_eq!(komihash(&content, 0), hash0, "content: {:?}, with default seed", content);
            assert_eq!(komihash(&content, seed), hash1, "content: {:?}, with seed {}", content, seed);
        }
    }

    #[test]
    fn streamed_hash_test() {
        for (hash0, hash1, seed, content) in test_vector() {
            let mut hasher = StreamedKomihash::new();
            hasher.write(&content);
            assert_eq!(hasher.finish(), hash0, "streamed hash content: {:?}, with default seed", content);

            let mut hasher = StreamedKomihash::new_with_seed(seed);
            hasher.write(&content);
            assert_eq!(hasher.finish(), hash1, "streamed hash content: {:?}, with seed: {}", content, seed);


            for size in 1..127 {
                let mut hasher = StreamedKomihash::new();
                let mut bytes: &[u8] = &content;
                while bytes.len() > 0 {
                    let slice = &bytes[..min(bytes.len(), size)];
                    bytes = &bytes[slice.len()..];
                    hasher.write(slice);
                }
                assert_eq!(hasher.finish(), hash0, "streamed hash content: {:?}, with default seed", content);


                let mut hasher = StreamedKomihash::new_with_seed(seed);
                let mut bytes: &[u8] = &content;

                while bytes.len() > 0 {
                    let slice = &bytes[..min(bytes.len(), size)];
                    bytes = &bytes[slice.len()..];
                    hasher.write(slice);
                }
                assert_eq!(hasher.finish(), hash1, "streamed hash content: {:?}, with seed: {}", content, seed);
            }
        }
    }

    #[test]
    fn rand_test() {
        for (seed0, seed1, test_vector) in komi_rand_test_vector() {
            let mut rand = Komirand::new(seed0, seed1);
            for value in test_vector {
                assert_eq!(rand.next(), value);
            }
        }
    }

    #[test]
    fn test_fill_bytes() {
        for (seed0, seed1, test_vector) in komi_rand_test_vector() {

            for buff_size in 1..96 {
                let mut rand = Komirand::new(seed0, seed1);
                let mut buff = Vec::<u8>::with_capacity(buff_size);
                buff.resize(buff_size, 0);
                rand.fill_bytes(&mut buff);
                let mut idx = 0;
                for chunk in buff.chunks(8) {
                    let mut tmp: [u8; 8] = [0; 8];
                    tmp[0..chunk.len()].copy_from_slice(chunk);
                    let value = read_word(&tmp).0;
                    assert_eq!(value,
                               test_vector[idx] & (0xffffffffffffffff >> (64 - chunk.len() * 8)),
                               "buff_size: {}, chunk.len(): {}, idx: {}, test_vector[idx]={}, tmp={:?}",
                               buff_size,
                               chunk.len(),
                               idx,
                               test_vector[idx],
                        tmp
                    );
                    idx += 1;
                }
            }
        }
    }
}
