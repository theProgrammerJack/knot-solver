use knot_solver::Knot;
use std::str::FromStr;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rand::Rng;

fn simulate(input: String) {
    let knot = Knot::from_str(input.as_str()).unwrap();
    let res = knot.resolutions();
}

fn gen_input(len: u8) -> String {
    let mut s = String::new();
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        s.push(rng.gen_range(65u8, 91u8) as char);
    }
    s
}

fn basic_benchmark(c: &mut Criterion) {
    c.bench_function("10 crossings", move |b| {
        b.iter_batched(
            || gen_input(10),
            |input| simulate(input),
            BatchSize::NumIterations(50),
        )
    });
    c.bench_function_over_inputs(
        "Various crossing quantities",
        move |b, input| b.iter(|| simulate(input.clone())),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            .iter()
            .map(|i| gen_input(*i))
            .collect::<Vec<String>>(),
    );
}

criterion_group!(benches, basic_benchmark);
criterion_main!(benches);
