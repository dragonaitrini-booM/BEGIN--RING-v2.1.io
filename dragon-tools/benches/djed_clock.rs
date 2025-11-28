use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dragon_tools::DjedClock;

pub fn phase_lock_benchmark(c: &mut Criterion) {
    let clock = DjedClock::new();

    c.bench_function("djed_phase_calc", |b| {
        b.iter(|| {
            black_box(clock.calculate_phase())
        })
    });
}

criterion_group!(benches, phase_lock_benchmark);
criterion_main!(benches);
