
use std::collections::{HashSet, BTreeSet};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fhq_treap::TreapSet;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("HashSet insert", |b| b.iter(|| {
        let mut h = BTreeSet::<u8>::new();
        for _ in 0..100 {
            h.insert(rand::random());
        }
    }));
    c.bench_function("TreapSet insert", |b| b.iter(|| {
        let mut h = TreapSet::<u8>::new();
        for _ in 0..100 {
            h.insert(rand::random());
        }
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
