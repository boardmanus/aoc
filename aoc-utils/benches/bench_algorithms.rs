use aoc_utils::grof::algorithms as grofalg;
use aoc_utils::grof::simple as grofsimp;

use criterion::{criterion_group, criterion_main, Criterion};

fn benchmarks(c: &mut Criterion) {
    let input = include_str!("input");
    let grof = grofsimp::SimpleGraphBuilder::parse("grof", input, "-").unwrap();
    println!("{grof}");

    let mut g = c.benchmark_group("clique");
    g.sampling_mode(criterion::SamplingMode::Flat);
    g.bench_function("grof: find_maximal_clique", |b| {
        b.iter(|| grofalg::find_maximal_clique(&grof, "dm"))
    });
    g.bench_function("grof: find_maximum_clique", |b| {
        b.iter(|| grofalg::find_maximum_clique(&grof))
    });
    g.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
