use criterion::{criterion_group, criterion_main, Criterion};
use dragon_tools::{AtomicWriter, AkashicRecord, PhaseAngle, DjedClock};
use tempfile::NamedTempFile;

pub fn chaos_boundary_benchmark(c: &mut Criterion) {
    let file = NamedTempFile::new().unwrap();
    let mut writer = AtomicWriter::new(file.path()).unwrap();

    let record = AkashicRecord::new(
        1,
        DjedClock::now_ns(),
        PhaseAngle::new(90.0),
        [0; 32],
        &[0xAA; 22]
    );

    c.bench_function("atomic_write_uring", |b| {
        b.iter(|| {
            writer.write_record(&record).unwrap();
        })
    });
}

criterion_group!(benches, chaos_boundary_benchmark);
criterion_main!(benches);
