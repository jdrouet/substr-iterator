use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use substr_iterator::SubstrIter;

const QUERIES: [&str; 4] = [
    "ab",
    "whatever",
    "hello world",
    "this is a long sentence that will be iterated over",
];

fn criterion_benchmark(c: &mut Criterion) {
    for query in QUERIES {
        c.bench_with_input(BenchmarkId::new("iterate", query), &query, |b, &query| {
            b.iter(|| SubstrIter::<3>::from(query).all(|item| item.len() == 3));
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
