#[macro_use]
extern crate criterion;
extern crate boolmonoms;

use boolmonoms::MonomBits;

use criterion::Criterion;

fn bench_mul() {
    let a = MonomBits::from(0);
    let b = MonomBits::from(1);

    let _res = a * b;
}
fn bench_div() {
    let ab = MonomBits::from(vec![0, 1]);
    let b = MonomBits::from(1);

    let _res = ab / b;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("a*b", |b| b.iter(|| bench_mul()));
    c.bench_function("a/b", |b| b.iter(|| bench_div()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
