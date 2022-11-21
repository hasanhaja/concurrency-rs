use core::{async_process_images, clear_dir, mult_process_images, seq_process_images};

use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

const SIGMAS: [f32; 10] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
const DEFAULT_SIGMA: f32 = 0.01;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input_path = "input-images";
    // -----Computation performance benchmark-----
    // https://github.com/bheisler/criterion.rs/issues/548
    for sigma in SIGMAS.iter() {
        c.bench_function(format!("Sequential ({})", sigma).as_str(), |b| {
            b.iter(|| {
                seq_process_images(input_path,"seq-output-images", *sigma).unwrap();
                clear_dir("seq-output-images").unwrap();
            })
        });

        c.bench_function(format!("Multi-threaded ({})", sigma).as_str(), |b| {
            b.iter(|| {
                mult_process_images(input_path, "mult-output-images", *sigma).unwrap();
                clear_dir("mult-output-images").unwrap();
            })
        });

        c.bench_function(format!("Async ({})", sigma).as_str(), |b| {
            b.to_async(Runtime::new().unwrap()).iter(|| async {
                async_process_images(input_path, "async-output-images", &sigma).await.unwrap();
                clear_dir("async-output-images").unwrap();
            })
        });
    }

    // -----IO Intensive benchmark-----
    let inputs = vec![25, 50, 75, 100, 125, 150, 175, 200, 225, 250];

    for n in inputs.iter() {
        c.bench_function(format!("IO Multi-threaded ({} images)", n).as_str(), |b| {
            b.iter(|| {
                mult_process_images(format!("{}-{}", input_path, n).as_str(), "mult-output-images",  DEFAULT_SIGMA).unwrap();
                clear_dir("mult-output-images").unwrap();
            })
        });

        c.bench_function(format!("IO Async ({} images)", n).as_str(), |b| {
            b.to_async(Runtime::new().unwrap()).iter(|| async {
                async_process_images(format!("{}-{}", input_path, n).as_str(), "async-output-images", &DEFAULT_SIGMA).await.unwrap();
                clear_dir("async-output-images").unwrap();
            })
        });

    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
