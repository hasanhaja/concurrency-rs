use core::{seq_process_images, clear_outputs, mult_process_images};

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let sigmas = vec![0.5, 3.0, 5.5];
    for sigma in &sigmas {
        c.bench_function(format!("Sequential ({})", sigma).as_str(), |b| b.iter(|| {
            seq_process_images(*sigma).unwrap();
            clear_outputs("seq-output-images").unwrap();
        }));
    }
    
    for sigma in &sigmas {
        c.bench_function(format!("Multi-threaded ({})", sigma).as_str(), |b| b.iter(|| {
            mult_process_images(*sigma).unwrap();
            clear_outputs("mult-output-images").unwrap();
        }));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);