use dragon_tools::{DjedClock, ProtocolStack, AtomicWriter, AkashicRecord};
use tempfile::TempDir;
use std::fs;

#[test]
fn test_full_commit_cycle() {
    // 1. Setup
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    let mut writer = AtomicWriter::new(base_path).unwrap();

    // 2. Execution
    let clock = DjedClock::new();
    let phase = clock.calculate_phase();

    let record = AkashicRecord::new(
        [0x11; 32],
        [0x22; 32],
        phase.to_u32(),
        12345,
        DjedClock::now_ns(),
    );

    let protocol = ProtocolStack::new();
    let hash = protocol.hash_record(&record);
    let write_result = writer.atomic_commit(&record);

    // 3. Verification
    assert!(write_result.is_ok(), "Write failed: {:?}", write_result.err());
    assert_ne!(hash, [0; 32]);

    // Verify sidecar files
    let work_path = base_path.join("work_register.dat");
    let hist_path = base_path.join("hist_register.dat");
    let witness_path = base_path.join("witness_register.dat");

    // Check sizes
    let work_meta = fs::metadata(&work_path).expect("Work file not found");
    assert_eq!(work_meta.len(), 128, "Work file size incorrect");

    let hist_meta = fs::metadata(&hist_path).expect("History file not found");
    assert_eq!(hist_meta.len(), 128, "History file size incorrect");

    let witness_meta = fs::metadata(&witness_path).expect("Witness file not found");
    assert_eq!(witness_meta.len(), 128, "Witness file size incorrect");
}
