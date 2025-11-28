use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dragon_tools::{ProtocolStack, AkashicRecord};

pub fn protocol_benchmark(c: &mut Criterion) {
    let protocol = ProtocolStack::new();
    let record = AkashicRecord::new(
        [1; 32],
        [2; 32],
        18000,
        1,
        1000,
    );

    c.bench_function("blake3_hash_record", |b| {
        b.iter(|| {
            protocol.hash_record(black_box(&record))
        })
    });
}

criterion_group!(benches, protocol_benchmark);
criterion_main!(benches);
