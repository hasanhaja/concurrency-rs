use core::seq_process_images;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sequential benchmark", |b| b.iter(|| seq_process_images()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);