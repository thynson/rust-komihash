#![feature(test)]

use std::hash::Hasher;
use std::ops::DerefMut;
use std::sync::Mutex;

extern crate test;
use test::Bencher;
use std::time::SystemTime;
use komihash::{Komirand, StreamedKomihash};


fn bench_template<const N: usize, const SIZE: usize>(b: &mut Bencher) {
    let mut content = [0u8; SIZE];
    let unix_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;
    let mut rand = Komirand::init(unix_timestamp);
    rand.fill_bytes(&mut content);
    let mut n = 0;
    b.iter(move || {
        let mut hasher = StreamedKomihash::new_with_seed(n);
        n += 1;
        for _ in 0..N {
            hasher.write(&content);
        }
        hasher.finish();
        n += 1;
    });
}

#[bench]
fn bench_streamed_with_4x64bytes_input(b: &mut Bencher) {
    bench_template::<4, 128>(b);
}

#[bench]
fn bench_streamed_with_8x128bytes_input(b: &mut Bencher) {
    bench_template::<8, 128>(b);

}

#[bench]
fn bench_streamed_with_16x256bytes_input(b: &mut Bencher) {
    bench_template::<16, 256>(b);
}

#[bench]
fn bench_streamed_with_32x512bytes_input(b: &mut Bencher) {
    bench_template::<32, 512>(b);
}

#[bench]
fn bench_streamed_with_64x1024bytes_input(b: &mut Bencher) {
    bench_template::<64, 1024>(b);
}


// When the buffer size can be aligned to 64 bytes, copy will be eliminated.
// So the situation that the buffer size is not aligned to 64 bytes is need to be benchmarked too,

#[bench]
fn bench_streamed_with_5x100bytes_input(b: &mut Bencher) {
    bench_template::<5, 100>(b);
}

#[bench]
fn bench_streamed_with_10x500bytes_input(b: &mut Bencher) {
    bench_template::<10, 500>(b);
}

#[bench]
fn bench_streamed_with_20x1000bytes_input(b: &mut Bencher) {
    bench_template::<20, 1000>(b);
}

#[bench]
fn bench_streamed_with_50x5000bytes_input(b: &mut Bencher) {
    bench_template::<50, 5000>(b);
}