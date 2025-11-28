use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dragon_tools::{ProtocolStack, AkashicRecord, PhaseAngle};

pub fn protocol_benchmark(c: &mut Criterion) {
    let protocol = ProtocolStack::new();
    let record = AkashicRecord::new(
        1,
        1000,
        PhaseAngle::new(180.0),
        [0; 32],
        &[0u8; 22]
    );

    c.bench_function("blake3_hash_record", |b| {
        b.iter(|| {
            protocol.hash_record(black_box(&record))
        })
    });
}

criterion_group!(benches, protocol_benchmark);
criterion_main!(benches);
