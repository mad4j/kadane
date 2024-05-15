use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kadane::kadane;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("kadane", |b| {
        b.iter(|| kadane(black_box(&[-2, 1, -3, 4, -1, 2, 1, -5, 4])))
    });
}

criterion_group!(benches, criterion_benchmark,);
criterion_main!(benches);
