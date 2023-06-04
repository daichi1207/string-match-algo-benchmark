use brute_force::brute_force;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn brute_force_benchmark(c: &mut Criterion) {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".repeat(10000);
    let pattern = "ipsum";
    c.bench_function("brute_force", |b| {
        b.iter(|| brute_force(black_box(&text), black_box(pattern)))
    });
}

criterion_group!(benches, brute_force_benchmark);
criterion_main!(benches);
