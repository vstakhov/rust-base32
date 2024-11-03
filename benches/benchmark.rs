
use rspamd_base32;

use criterion::{black_box, criterion_group, criterion_main,
                BenchmarkId, Bencher, Criterion, Throughput};
use rspamd_base32::{encode, decode, encode_alphabet_slice};
use rspamd_base32::alphabet::ZBASE32;
use rspamd_base32::encode::encoded_len;
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

fn do_encode_bench(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size * 5 / 8);
    fill_buf(&mut v);
    let mut buf = vec![0; encoded_len(v.len()).expect("bad size")];

    b.iter(|| {
        encode_alphabet_slice(&v, buf.as_mut_slice(), &ZBASE32);
        black_box(&buf);
    });
}

const SIZES: [usize; 5] = [10, 128, 1024, 12400, 1024 * 1024 * 2];

fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_decode");
    for sz in SIZES.iter() {
        group.throughput(Throughput::Bytes(*sz as u64));
        group.bench_with_input(BenchmarkId::new("decode bench", sz),
                           sz,
                           do_decode_bench);
    }
}

fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_encode");
    for sz in SIZES.iter() {
        group.throughput(Throughput::Bytes(*sz as u64));
        group.bench_with_input(BenchmarkId::new("encode bench", sz),
                               sz,
                               do_encode_bench);
    }
}

criterion_group!(benches, bench_decode, bench_encode);
criterion_main!(benches);
