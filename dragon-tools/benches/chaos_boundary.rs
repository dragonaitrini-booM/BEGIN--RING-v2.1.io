use criterion::{criterion_group, criterion_main, Criterion};
use dragon_tools::{AtomicWriter, AkashicRecord, DjedClock};
use tempfile::TempDir;

pub fn chaos_boundary_benchmark(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let mut writer = AtomicWriter::new(temp_dir.path()).unwrap();

    let record = AkashicRecord::new(
        [0xAA; 32],
        [0xBB; 32],
        9000,
        1,
        DjedClock::now_ns(),
    );

    c.bench_function("atomic_write_uring", |b| {
        b.iter(|| {
            writer.atomic_commit(&record).unwrap();
        })
    });
}

criterion_group!(benches, chaos_boundary_benchmark);
criterion_main!(benches);
