use clausen::Cl;
use criterion::*;

fn bench_cl1(c: &mut Criterion) {
    let mut group = c.benchmark_group("cl(1,x)");
    group.bench_function("x=-0.1", |b| b.iter(|| black_box(-0.1).cl(1)));
    group.bench_function("x=-3.2", |b| b.iter(|| black_box(-3.2).cl(1)));
    group.bench_function("x=-6.5", |b| b.iter(|| black_box(-6.5).cl(1)));
    group.bench_function("x=0.1" , |b| b.iter(|| black_box( 0.1).cl(1)));
    group.bench_function("x=3.2" , |b| b.iter(|| black_box( 3.2).cl(1)));
    group.bench_function("x=6.5" , |b| b.iter(|| black_box( 6.5).cl(1)));
    group.finish();
}

fn bench_cl2(c: &mut Criterion) {
    let mut group = c.benchmark_group("cl(2,x)");
    group.bench_function("x=-0.1", |b| b.iter(|| black_box(-0.1).cl(2)));
    group.bench_function("x=-3.2", |b| b.iter(|| black_box(-3.2).cl(2)));
    group.bench_function("x=-6.5", |b| b.iter(|| black_box(-6.5).cl(2)));
    group.bench_function("x=0.1" , |b| b.iter(|| black_box( 0.1).cl(2)));
    group.bench_function("x=3.2" , |b| b.iter(|| black_box( 3.2).cl(2)));
    group.bench_function("x=6.5" , |b| b.iter(|| black_box( 6.5).cl(2)));
    group.finish();
}

criterion_group!(benches, bench_cl1, bench_cl2);
criterion_main!(benches);
