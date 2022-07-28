use std::borrow::BorrowMut;
use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard};
use crate::{komi_hash};

extern crate test;

use test::Bencher;
use pcg::Pcg;
use rand_core::RngCore;



lazy_static! {
    static ref RNG: Mutex<Pcg> = Mutex::new(Pcg::default());
}


fn bench_template<const SIZE: usize>(b: &mut Bencher) {
    let mut content = [0u8; SIZE];
    RNG.lock().unwrap().deref_mut().fill_bytes(&mut content);
    let mut n = 0;
    b.iter(move || {
        komi_hash(&content, n);
        n += 1;
    });
}

#[bench]
fn bench_oneshot_with_4bytes_input(b: &mut Bencher) {
    bench_template::<4>(b);
}

#[bench]
fn bench_oneshot_with_8bytes_input(b: &mut Bencher) {
    bench_template::<8>(b);
}

#[bench]
fn bench_oneshot_with_16bytes_input(b: &mut Bencher) {
    bench_template::<16>(b);
}

#[bench]
fn bench_oneshot_with_64bytes_input(b: &mut Bencher) {
    bench_template::<64>(b);
}

#[bench]
fn bench_oneshot_with_256bytes_input(b: &mut Bencher) {
    bench_template::<256>(b);
}

#[bench]
fn bench_oneshot_with_1024bytes_input(b: &mut Bencher) {
    bench_template::<1024>(b);
}

#[bench]
fn bench_oneshot_with_4096bytes_input(b: &mut Bencher) {
    bench_template::<4096>(b);
}

#[bench]
fn bench_oneshot_with_16384bytes_input(b: &mut Bencher) {
    bench_template::<16384>(b);
}

#[bench]
fn bench_oneshot_with_65536bytes_input(b: &mut Bencher) {
    bench_template::<65536>(b);
}
