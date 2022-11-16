use core::{async_process_images, clear_outputs, mult_process_images, seq_process_images};

use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

const SIGMAS: [f32; 3] = [0.5, 3.0, 5.5];

pub fn criterion_benchmark(c: &mut Criterion) {
    // https://github.com/bheisler/criterion.rs/issues/548
    for sigma in SIGMAS.iter() {
        c.bench_function(format!("Sequential ({})", sigma).as_str(), |b| {
            b.iter(|| {
                seq_process_images(*sigma).unwrap();
                clear_outputs("seq-output-images").unwrap();
            })
        });

        c.bench_function(format!("Multi-threaded ({})", sigma).as_str(), |b| {
            b.iter(|| {
                mult_process_images(*sigma).unwrap();
                clear_outputs("mult-output-images").unwrap();
            })
        });

        c.bench_function(format!("Async ({})", sigma).as_str(), |b| {
            b.to_async(Runtime::new().unwrap()).iter(|| async {
                async_process_images(&sigma).await.unwrap();
                clear_outputs("async-output-images").unwrap();
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
