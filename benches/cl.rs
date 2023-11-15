use clausen::Cl;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_cl2(c: &mut Criterion) {
    c.bench_function("cl(2, -0.1)", |b| b.iter(|| black_box(-0.1).cl(2)));
    c.bench_function("cl(2, -3.2)", |b| b.iter(|| black_box(-3.2).cl(2)));
    c.bench_function("cl(2, -6.5)", |b| b.iter(|| black_box(-6.5).cl(2)));
    c.bench_function("cl(2,  0.1)", |b| b.iter(|| black_box( 0.1).cl(2)));
    c.bench_function("cl(2,  3.2)", |b| b.iter(|| black_box( 3.2).cl(2)));
    c.bench_function("cl(2,  6.5)", |b| b.iter(|| black_box( 6.5).cl(2)));
}

criterion_group!(benches, bench_cl2);
criterion_main!(benches);
