use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pythia_hft_fixedpoint::FixedPoint;

type FP16 = FixedPoint<16>;

fn bench_add(c: &mut Criterion) {
    let a = FP16::from_int(1000);
    let b = FP16::from_int(2000);
    c.bench_function("fp_add", |bencher| {
        bencher.iter(|| black_box(a).add(black_box(b)))
    });
}

fn bench_mul(c: &mut Criterion) {
    let a = FP16::from_raw(3 * (1 << 16) + 12345);
    let b = FP16::from_raw(7 * (1 << 16) + 54321);
    c.bench_function("fp_mul", |bencher| {
        bencher.iter(|| black_box(a).mul(black_box(b)))
    });
}

fn bench_cmp(c: &mut Criterion) {
    let a = FP16::from_int(100);
    let b = FP16::from_int(200);
    c.bench_function("fp_cmp", |bencher| {
        bencher.iter(|| black_box(a).cmp_fp(black_box(b)))
    });
}

criterion_group!(benches, bench_add, bench_mul, bench_cmp);
criterion_main!(benches);
