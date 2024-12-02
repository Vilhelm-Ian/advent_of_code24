use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[path = "../src/bin/part_2.rs"]
mod part_2;

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/bin/input.txt");
    c.bench_function("fib 20", |b| {
        b.iter(|| part_2::solve(input));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
