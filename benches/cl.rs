use clausen::Cl;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn cl2(x: f64) {
    x.cl(2);
}

fn bench_cl2(c: &mut Criterion) {
    c.bench_function("cl(2, -0.1)", |b| b.iter(|| cl2(black_box(-0.1))));
    c.bench_function("cl(2, -3.2)", |b| b.iter(|| cl2(black_box(-3.2))));
    c.bench_function("cl(2, -6.5)", |b| b.iter(|| cl2(black_box(-6.5))));
    c.bench_function("cl(2,  0.1)", |b| b.iter(|| cl2(black_box( 0.1))));
    c.bench_function("cl(2,  3.2)", |b| b.iter(|| cl2(black_box( 3.2))));
    c.bench_function("cl(2,  6.5)", |b| b.iter(|| cl2(black_box( 6.5))));
}

criterion_group!(benches, bench_cl2);
criterion_main!(benches);
