#[macro_use]
extern crate criterion;

use std::sync::RwLock;
use criterion::Criterion;
use criterion::black_box;

#[macro_use]
use qwutils::*;

lazy_static::lazy_static! {
    pub static ref SLOW: RwLock<Vec<u64>> = RwLock::default();
    pub static ref ULTRA: Vec<u64> = vec![8;256];
}

create_static_stor!(pub FAST: Vec<u64>);

#[inline(never)]
fn access_rwlock(i: usize) -> u64 {
    SLOW.read().unwrap()[0]
}

#[inline(never)]
fn access_fast(i: usize) -> u64 {
    FAST::with(|i| i[0] )
}

fn criterion_benchmark(c: &mut Criterion) {
    {
        *SLOW.write().unwrap() = vec![8;256];
        FAST::with_mut(|i| *i=vec![8;256] );
    }

    c.bench_function("access rwlock", |b| {
        b.iter(|| black_box(SLOW.read().unwrap()[0]))
    });
    c.bench_function("access oof", |b| {
        b.iter(|| black_box(FAST::with(|i| i[0] )))
    });
    c.bench_function("access ultra", |b| {
        b.iter(|| black_box(ULTRA[0]))
    });
    c.bench_function("access hyper", |b| {
        let i: u64 = 20;
        b.iter(|| black_box(i))
    });
    c.bench_function("access HAIPA HAIPA", |b| {
        b.iter(|| black_box(666))
    });
    c.bench_function("access ultra", |b| {
        b.iter(|| black_box(ULTRA[255]))
    });
    c.bench_function("access oof", |b| {
        b.iter(|| black_box(FAST::with(|i| i[255] )))
    });
    c.bench_function("access rwlock", |b| {
        b.iter(|| black_box(SLOW.read().unwrap()[255]))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
