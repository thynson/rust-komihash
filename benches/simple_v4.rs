#![feature(test)]

extern crate test;

use komihash::v4::{komihash, Komirand};
use std::hint::black_box;
use std::time::SystemTime;
use test::Bencher;

fn bench_template<const N: u64, const SIZE: usize>(b: &mut Bencher) {
    let mut content = [0u8; SIZE];
    let unix_timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let mut rand = Komirand::init(unix_timestamp);
    rand.fill_bytes(&mut content);
    let mut n = 10;
    let mut ret = 0u64;

    b.iter(|| {
        for _ in 0..N {
            ret ^= komihash(&content, n);
        }
        n += N;
    });
    black_box(ret);
}

#[bench]
fn bench_simple_with_00000bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 0>(b);
}
#[bench]
fn bench_simple_with_00001bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 1>(b);
}

#[bench]
fn bench_simple_with_00002bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 2>(b);
}

#[bench]
fn bench_simple_with_00003bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 3>(b);
}

#[bench]
fn bench_simple_with_00004bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 4>(b);
}

#[bench]
fn bench_simple_with_00005bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 5>(b);
}

#[bench]
fn bench_simple_with_00006bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 6>(b);
}

#[bench]
fn bench_simple_with_00007bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 7>(b);
}
#[bench]
fn bench_simple_with_00008bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 8>(b);
}

#[bench]
fn bench_simple_with_00009bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 9>(b);
}

#[bench]
fn bench_simple_with_00010bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 10>(b);
}

#[bench]
fn bench_simple_with_00011bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 11>(b);
}

#[bench]
fn bench_simple_with_00012bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 12>(b);
}

#[bench]
fn bench_simple_with_00013bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 13>(b);
}

#[bench]
fn bench_simple_with_00014bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 10>(b);
}

#[bench]
fn bench_simple_with_00015bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 10>(b);
}

#[bench]
fn bench_simple_with_00016bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 16>(b);
}

#[bench]
fn bench_simple_with_00024bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 24>(b);
}

#[bench]
fn bench_simple_with_00032bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 32>(b);
}

#[bench]
fn bench_simple_with_00048bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 48>(b);
}

#[bench]
fn bench_simple_with_00064bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 64>(b);
}

#[bench]
fn bench_simple_with_00096bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 96>(b);
}

#[bench]
fn bench_simple_with_00128bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 128>(b);
}

#[bench]
fn bench_simple_with_00192bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 192>(b);
}

#[bench]
fn bench_simple_with_00256bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 256>(b);
}

#[bench]
fn bench_simple_with_01024bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 1024>(b);
}

#[bench]
fn bench_simple_with_04096bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 4096>(b);
}

#[bench]
fn bench_simple_with_16384bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 16384>(b);
}

#[bench]
fn bench_simple_with_65536bytes_x10_input(b: &mut Bencher) {
    bench_template::<10, 65536>(b);
}
