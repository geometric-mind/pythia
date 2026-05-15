use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pythia_hft_marketmaking::AvellanedaStoikov;

fn bench_quote(c: &mut Criterion) {
    let mm = AvellanedaStoikov::new(0.1, 0.02);
    c.bench_function("as_quote", |b| {
        b.iter(|| mm.quote(black_box(100.0), black_box(500.0), black_box(1.5)))
    });
}

fn bench_inventory_risk(c: &mut Criterion) {
    let mm = AvellanedaStoikov::new(0.1, 0.02);
    c.bench_function("inventory_risk", |b| {
        b.iter(|| mm.inventory_risk(black_box(500.0)))
    });
}

criterion_group!(benches, bench_quote, bench_inventory_risk);
criterion_main!(benches);
