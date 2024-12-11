use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[path = "../src/bin/part_2.rs"]
mod part_2;

#[path = "../src/bin/part_1.rs"]
mod part_1;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/bin/input.txt");
    c.bench_function("part_2", |b| {
        b.iter(|| part_2::solve(input, 75));
    });

    c.bench_function("part_1", |b| {
        b.iter(|| part_1::solve(input, 25));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
