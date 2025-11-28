use dragon_tools::{DjedClock, ProtocolStack, AtomicWriter, AkashicRecord};
use tempfile::NamedTempFile;
use std::fs;

#[test]
fn test_full_commit_cycle() {
    // 1. Setup
    // Use NamedTempFile to get a path, but we will look for sidecar files.
    let file = NamedTempFile::new().unwrap();
    let base_path = file.path();
    // We need to keep the temp file object alive so the directory is valid (if it was a temp dir, but here it's a file in tmp).
    // Actually NamedTempFile deletes the file on drop.
    // The sidecar files are created in the same directory.

    let mut writer = AtomicWriter::new(base_path).unwrap();

    // 2. Execution
    let phase = clock_phase_helper();
    let record = AkashicRecord::new(
        12345,
        DjedClock::now_ns(),
        phase,
        [0; 32],
        b"Integration Test",
    );

    let protocol = ProtocolStack::new();
    let hash = protocol.hash_record(&record);
    let write_result = writer.write_record(&record);

    // 3. Verification
    assert!(write_result.is_ok(), "Write failed: {:?}", write_result.err());
    assert_ne!(hash, [0; 32]);
    assert!(record.phase >= 0.0 && record.phase < 360.0);

    // Verify sidecar files
    let file_name = base_path.file_name().unwrap().to_string_lossy();
    let dir = base_path.parent().unwrap();

    let work_path = dir.join(format!("{}_work.log", file_name));
    let hist_path = dir.join(format!("{}_hist.log", file_name));
    let wit_path = dir.join(format!("{}_wit.log", file_name));

    // Check sizes
    let work_meta = fs::metadata(&work_path).expect("Work file not found");
    assert_eq!(work_meta.len(), 128, "Work file size incorrect");

    let hist_meta = fs::metadata(&hist_path).expect("History file not found");
    assert_eq!(hist_meta.len(), 128, "History file size incorrect");

    let wit_meta = fs::metadata(&wit_path).expect("Witness file not found");
    assert_eq!(wit_meta.len(), 128, "Witness file size incorrect");

    // Cleanup manually (tempfile only cleans the base file)
    let _ = fs::remove_file(work_path);
    let _ = fs::remove_file(hist_path);
    let _ = fs::remove_file(wit_path);
}

fn clock_phase_helper() -> dragon_tools::PhaseAngle {
    let clock = DjedClock::new();
    clock.calculate_phase()
}
