extern crate criterion;
extern crate rand;
extern crate base32;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Bencher, Criterion};
use base32::{encode, decode};
use rand::prelude::*;

fn fill_buf(v: &mut Vec<u8>) {
    let capacity = v.capacity();
    let mut r = rand::rngs::StdRng::from_entropy();

    while v.len() < capacity {
        v.push(r.gen::<u8>());
    }
}

fn do_decode_bench(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size * 5 / 8);
    fill_buf(&mut v);
    let encoded = encode(&v);

    b.iter(|| {
        let orig = decode(&encoded);
        black_box(&orig);
    });
}

const SIZES: [usize; 5] = [10, 128, 1024, 12400, 1024 * 1024 * 2];

fn bench_decode(c: &mut Criterion) {
    for sz in SIZES {
        c.bench_with_input(BenchmarkId::new("decode bench", sz),
                           &sz,
                           do_decode_bench);
    }
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);
