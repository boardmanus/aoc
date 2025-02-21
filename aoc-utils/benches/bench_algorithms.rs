use aoc_utils::grif::simple_vec as sv;
use aoc_utils::grif::{
    algorithms::{find_maximal_clique, find_maximum_clique},
    simple::SimpleGraphBuilder,
};
use aoc_utils::grof::algorithms as grofalg;
use aoc_utils::grof::simple as grofsimp;

use criterion::{criterion_group, criterion_main, Criterion};

fn benchmarks(c: &mut Criterion) {
    let input = include_str!("input");
    let graph = SimpleGraphBuilder::parse("bench", input, "-").unwrap();
    let graph_vec = sv::SimpleGraphBuilder::parse("bench", input, "-").unwrap();
    let grof = grofsimp::SimpleGraphBuilder::parse("grof", input, "-").unwrap();
    println!("{graph}");
    println!("{grof}");

    let mut g = c.benchmark_group("clique");
    g.sampling_mode(criterion::SamplingMode::Flat);
    g.bench_function("find_maximal_clique", |b| {
        b.iter(|| find_maximal_clique(&graph, "dm"))
    });
    g.bench_function("find_maximum_clique", |b| {
        b.iter(|| find_maximum_clique(&graph))
    });
    g.bench_function("vec: find_maximal_clique", |b| {
        b.iter(|| find_maximal_clique(&graph_vec, "dm"))
    });
    g.bench_function("vec: find_maximum_clique", |b| {
        b.iter(|| find_maximum_clique(&graph_vec))
    });
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
