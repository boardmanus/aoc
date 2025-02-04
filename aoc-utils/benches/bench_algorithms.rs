use aoc_utils::grif::{
    algorithms::{find_maximal_clique, find_maximum_clique},
    simple::SimpleGraphBuilder,
};
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmarks(c: &mut Criterion) {
    let graph = SimpleGraphBuilder::parse(include_str!("input"), "-").unwrap();
    println!("{graph}");

    let mut g = c.benchmark_group("clique");
    g.sampling_mode(criterion::SamplingMode::Flat);
    g.bench_function("find_maximal_clique", |b| {
        b.iter(|| find_maximal_clique(&graph, "dm"))
    });
    g.bench_function("find_maximum_clique", |b| {
        b.iter(|| find_maximum_clique(&graph))
    });
    g.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
