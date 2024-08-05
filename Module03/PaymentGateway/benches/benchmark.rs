use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn payment_benchmark(c: &mut Criterion) {
    c.bench_function("Payment Processing", |b| b.iter(|| process_payment()));
}

criterion_group!(benches, payment_benchmark);
criterion_main!(benches);