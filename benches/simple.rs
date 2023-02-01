#![feature(test)]

use std::ops::DerefMut;
use std::sync::Mutex;

extern crate test;

use test::Bencher;
use komihash::{komihash, Komirand};
use std::time::SystemTime;

fn bench_template<const SIZE: usize>(b: &mut Bencher) {
    let mut content = [0u8; SIZE];
    let unix_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;
    let mut rand = Komirand::init(unix_timestamp);
    rand.fill_bytes(&mut content);
    let mut n = 0;
    b.iter(move || {
        komihash(&content, n);
        n += 1;
    });
}

#[bench]
fn bench_simple_with_4bytes_input(b: &mut Bencher) {
    bench_template::<4>(b);
}

#[bench]
fn bench_simple_with_8bytes_input(b: &mut Bencher) {
    bench_template::<8>(b);
}

#[bench]
fn bench_simple_with_16bytes_input(b: &mut Bencher) {
    bench_template::<16>(b);
}

#[bench]
fn bench_simple_with_64bytes_input(b: &mut Bencher) {
    bench_template::<64>(b);
}

#[bench]
fn bench_simple_with_256bytes_input(b: &mut Bencher) {
    bench_template::<256>(b);
}

#[bench]
fn bench_simple_with_1024bytes_input(b: &mut Bencher) {
    bench_template::<1024>(b);
}

#[bench]
fn bench_simple_with_4096bytes_input(b: &mut Bencher) {
    bench_template::<4096>(b);
}

#[bench]
fn bench_simple_with_16384bytes_input(b: &mut Bencher) {
    bench_template::<16384>(b);
}

#[bench]
fn bench_simple_with_65536bytes_input(b: &mut Bencher) {
    bench_template::<65536>(b);
}
