use boyer_moore::boyer_moore;
use brute_force::brute_force;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kmp::knuth_morris_pratt;
use lorem_ipsum::generate;
use rabin_karp::rabin_karp;

fn boyer_moore_benchmark(c: &mut Criterion) {
    let text = generate(1000000).unwrap();
    let pattern = "ipsum";
    c.bench_function("boyer_moore", |b| {
        b.iter(|| boyer_moore(black_box(&text), black_box(pattern)))
    });
}

fn brute_force_benchmark(c: &mut Criterion) {
    let text = generate(1000000).unwrap();
    let pattern = "ipsum";
    c.bench_function("brute_force", |b| {
        b.iter(|| brute_force(black_box(&text), black_box(pattern)))
    });
}

fn knuth_morris_pratt_benchmark(c: &mut Criterion) {
    let text = generate(1000000).unwrap();
    let pattern = "ipsum";
    c.bench_function("knuth_morris_pratt", |b| {
        b.iter(|| knuth_morris_pratt(black_box(&text), black_box(pattern)))
    });
}

fn rabin_karp_benchmark(c: &mut Criterion) {
    let text = generate(1000000).unwrap();
    let pattern = "ipsum";
    c.bench_function("rabin_karp", |b| {
        b.iter(|| rabin_karp(black_box(&text), black_box(pattern)))
    });
}

criterion_group!(
    benches,
    boyer_moore_benchmark,
    brute_force_benchmark,
    knuth_morris_pratt_benchmark,
    rabin_karp_benchmark,
);
criterion_main!(benches);
