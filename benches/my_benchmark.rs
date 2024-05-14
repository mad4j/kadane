use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kadane::{kadane_00, kadane_04};

fn criterion_benchmark_00(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    //let result = kadane_00(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
    c.bench_function("kadane_00", |b| {
        b.iter(|| kadane_00(black_box(&[-2, 1, -3, 4, -1, 2, 1, -5, 4])))
    });
}

fn criterion_benchmark_04(c: &mut Criterion) {
    c.bench_function("kadane_04", |b| {
        b.iter(|| kadane_04(black_box(&[-2, 1, -3, 4, -1, 2, 1, -5, 4])))
    });
}

criterion_group!(benches, criterion_benchmark_00, criterion_benchmark_04);
criterion_main!(benches);
