use clausen::Sl;
use criterion::*;

fn bench_sl(c: &mut Criterion) {
    let mut group = c.benchmark_group("sl(n,1.0)");
    for n in 1..=30 {
        group.bench_function(format!("n={}", n), |b| b.iter(|| black_box(1.0).sl(n)));
    }
    group.finish();
}

criterion_group!(benches, bench_sl);
criterion_main!(benches);
