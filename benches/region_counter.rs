use knot_solver::RegionCounter;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

fn simulate(crossings: usize, regions: usize) {
    let mut counter = RegionCounter::new(regions);

    let mut rng = rand::thread_rng();

    for _ in 0..crossings {
        let uno = rng.gen_range(1usize, regions);
        let dos = rng.gen_range(1usize, regions);

        counter.combine(uno, dos);
    }
}

fn basic_benchmark(c: &mut Criterion) {
    c.bench_function("basics 1", |b| b.iter(|| simulate(150, 100)));

    c.bench_function("basics 2", |b| b.iter(|| simulate(150, 250)));

    c.bench_function("rand test", |b| {
        let mut rng = rand::thread_rng();

        b.iter(move || {
            rng.gen_range(1usize, 250)
        })
    });
}

criterion_group!(benches, basic_benchmark);
criterion_main!(benches);
