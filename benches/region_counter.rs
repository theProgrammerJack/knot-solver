use knot_solver::RegionCounter;

use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use rand::Rng;

fn basic_benchmark(c: &mut Criterion) {
    c.bench_function("basics", |b| b.iter(|| {
        let mut counter = RegionCounter::new(100);

        let mut rng = rand::thread_rng();

        for _ in 0..150 {
            let uno = rng.gen_range(1usize, 100);
            let dos = rng.gen_range(1usize, 100);

            counter.combine(uno, dos);
        }
    }));
}

criterion_group!(benches, basic_benchmark);
criterion_main!(benches);