use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use parsers::*;
use std::fs;

fn bench_vanilla_instructions_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vanilla");
    let s = fs::read_to_string("input/05").unwrap();
    group.bench_function(BenchmarkId::new("as_bytes", "05"), |b| {
        b.iter(|| parse_vanilla(s.as_str()))
    });
    group.bench_function(BenchmarkId::new("split", "05"), |b| {
        b.iter(|| parse_vanilla_slow(s.as_str()))
    });
    group.finish();
}

criterion_group!(vanilla_benches, bench_vanilla_instructions_parse);
criterion_main!(vanilla_benches);
