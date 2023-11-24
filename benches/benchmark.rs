use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use parsers::*;
use std::fs;

fn bench_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parsers");
    let s = fs::read_to_string("input/05").unwrap();
    group.bench_function(BenchmarkId::new("Nom", "05"), |b| {
        b.iter(|| parse_nom(s.as_str()))
    });
    group.bench_function(BenchmarkId::new("Pest", "05"), |b| {
        b.iter(|| parse_pest(s.as_str()))
    });
    group.bench_function(BenchmarkId::new("Vanilla", "05"), |b| {
        b.iter(|| parse_vanilla_slow(s.as_str()))
    });
    group.finish();
}

fn bench_parsers_fast_vanilla(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parsers2");
    let s = fs::read_to_string("input/05").unwrap();
    group.bench_function(BenchmarkId::new("Nom", "05"), |b| {
        b.iter(|| parse_nom(s.as_str()))
    });
    group.bench_function(BenchmarkId::new("Pest", "05"), |b| {
        b.iter(|| parse_pest(s.as_str()))
    });
    group.bench_function(BenchmarkId::new("Vanilla fast", "05"), |b| {
        b.iter(|| parse_vanilla(s.as_str()))
    });
    group.finish();
}

fn bench_simple_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parsers simple");
    let s = fs::read_to_string("input/05").unwrap();
    group.bench_function(BenchmarkId::new("Nom", "05"), |b| {
        b.iter(|| parse_simple_nom(s.as_str()))
    });
    group.bench_function(BenchmarkId::new("Pest", "05"), |b| {
        b.iter(|| parse_simple_pest(s.as_str()))
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_parsers,
    bench_parsers_fast_vanilla,
    bench_simple_parsers,
);
criterion_main!(benches);
