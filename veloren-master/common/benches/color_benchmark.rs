use criterion::{Criterion, black_box, criterion_group, criterion_main};

use vek::*;
use COPING CHRONICLES_common::util::{linear_to_srgb, srgb_to_linear_fast};

fn criterion_benchmark(c: &mut Criterion) {
    let mut c = c.benchmark_group("color");
    c.bench_function("srgb to linear (0.5, 0.1, 0.5)", |b| {
        b.iter(|| {
            black_box(srgb_to_linear_fast(black_box(Rgb::new(0.5, 0.1, 0.5))));
        })
    });
    c.bench_function("linear to srgb (0.5, 0.1, 0.5)", |b| {
        b.iter(|| {
            black_box(linear_to_srgb(black_box(Rgb::new(0.5, 0.1, 0.5))));
        })
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
