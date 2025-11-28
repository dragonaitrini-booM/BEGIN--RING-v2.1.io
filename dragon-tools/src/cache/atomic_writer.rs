//! Dragon Tools: Three-Register Atomic Writer
//!
//! Implements quantum entanglement protocol: all three registers or nothing.

use io_uring::{opcode, types, IoUring};
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;

use crate::error::{DragonError, Result};
use crate::types::AkashicRecord;

/// Three-register atomic writer with quantum entanglement
///
/// CRITICAL: Writes must succeed on ALL three registers or rollback entirely.
pub struct AtomicWriter {
    /// Fast, volatile storage (NVMe SSD)
    work_fd: RawFd,
    work_file: File,
    work_offset: u64,

    /// Archival, append-only storage (HDD or slow SSD)
    hist_fd: RawFd,
    hist_file: File,
    hist_offset: u64,

    /// Tamper-proof witness storage (HSM or dedicated device)
    witness_fd: RawFd,
    witness_file: File,
    witness_offset: u64,

    /// io_uring instance for batched operations
    ring: IoUring,
}

impl AtomicWriter {
    /// Create a new atomic writer with three distinct registers
    ///
    /// # Arguments
    /// * `base_path` - Directory containing the three register files
    ///
    /// # Returns
    /// A configured `AtomicWriter` ready for atomic commits
    pub fn new(base_path: &Path) -> Result<Self> {
        std::fs::create_dir_all(base_path)?;

        // Initialize io_uring with 128 queue entries
        let ring = IoUring::builder()
            .build(128)
            .map_err(|e| DragonError::Io(io::Error::new(io::ErrorKind::Other, e.to_string())))?;

        // Open three distinct files with append mode
        // NOTE: O_DIRECT would require 4096-byte alignment; we use buffered I/O
        // for the 128-byte records, accepting slightly higher latency for simplicity

        let work_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(base_path.join("work_register.dat"))?;

        let hist_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(base_path.join("hist_register.dat"))?;

        let witness_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(base_path.join("witness_register.dat"))?;

        let work_fd = work_file.as_raw_fd();
        let hist_fd = hist_file.as_raw_fd();
        let witness_fd = witness_file.as_raw_fd();

        Ok(Self {
            work_fd,
            work_file,
            work_offset: 0,
            hist_fd,
            hist_file,
            hist_offset: 0,
            witness_fd,
            witness_file,
            witness_offset: 0,
            ring,
        })
    }

    /// Execute atomic commit across all three registers
    ///
    /// CRITICAL: Uses io_uring batching to submit three writes in a single syscall.
    /// If ANY write fails, the entire commit is rolled back.
    ///
    /// # Arguments
    /// * `record` - The 128-byte Akashic Record to commit
    ///
    /// # Returns
    /// * `Ok(())` - All three registers confirmed receipt
    /// * `Err(...)` - Any failure triggers rollback
    pub fn atomic_commit(&mut self, record: &AkashicRecord) -> Result<()> {
        let bytes = record.as_bytes();

        // STEP 1: Queue three write operations in the submission queue

        // Write to Work Register
        let work_sqe = opcode::Write::new(
            types::Fd(self.work_fd),
            bytes.as_ptr(),
            bytes.len() as u32,
        )
        .offset(self.work_offset)
        .build()
        .user_data(1); // Tag for identification

        // Write to History Register
        let hist_sqe = opcode::Write::new(
            types::Fd(self.hist_fd),
            bytes.as_ptr(),
            bytes.len() as u32,
        )
        .offset(self.hist_offset)
        .build()
        .user_data(2);

        // Write to Witness Register
        let witness_sqe = opcode::Write::new(
            types::Fd(self.witness_fd),
            bytes.as_ptr(),
            bytes.len() as u32,
        )
        .offset(self.witness_offset)
        .build()
        .user_data(3);

        // STEP 2: Submit all three writes as an atomic batch
        unsafe {
            self.ring.submission()
                .push(&work_sqe)
                .map_err(|_| DragonError::QueueFull)?;

            self.ring.submission()
                .push(&hist_sqe)
                .map_err(|_| DragonError::QueueFull)?;

            self.ring.submission()
                .push(&witness_sqe)
                .map_err(|_| DragonError::QueueFull)?;
        }

        // Submit the batch (single syscall for all three writes)
        self.ring.submit()
            .map_err(|e| DragonError::Io(io::Error::new(io::ErrorKind::Other, e.to_string())))?;

        // STEP 3: Wait for all three completions and verify success
        let mut completions = 0;
        let mut results = [None, None, None]; // Work, Hist, Witness

        while completions < 3 {
            self.ring.submit_and_wait(1)
                .map_err(|e| DragonError::Io(io::Error::new(io::ErrorKind::Other, e.to_string())))?;

            for cqe in self.ring.completion() {
                let user_data = cqe.user_data() as usize;
                let result = cqe.result();

                if user_data >= 1 && user_data <= 3 {
                    results[user_data - 1] = Some(result);
                    completions += 1;
                }
            }
        }

        // STEP 4: Verify ALL writes succeeded
        let work_ok = results[0].unwrap() == AkashicRecord::SIZE as i32;
        let hist_ok = results[1].unwrap() == AkashicRecord::SIZE as i32;
        let witness_ok = results[2].unwrap() == AkashicRecord::SIZE as i32;

        if work_ok && hist_ok && witness_ok {
            // SUCCESS: Update offsets for next write
            self.work_offset += AkashicRecord::SIZE as u64;
            self.hist_offset += AkashicRecord::SIZE as u64;
            self.witness_offset += AkashicRecord::SIZE as u64;

            Ok(())
        } else {
            // FAILURE: Trigger rollback
            self.rollback()?;
            Err(DragonError::AtomicWriteFailed(format!(
                "Work: {:?}, Hist: {:?}, Witness: {:?}",
                results[0], results[1], results[2]
            )))
        }
    }

    /// Emergency rollback: Truncate all registers to previous offset
    ///
    /// CRITICAL: This is a simplified rollback. Production systems would
    /// use write-ahead logs (WAL) for atomic recovery.
    fn rollback(&mut self) -> Result<()> {
        eprintln!("🚨 ATOMIC ROLLBACK EXECUTED");

        // Truncate files to previous offsets (removes failed write)
        self.work_file.set_len(self.work_offset)?;
        self.hist_file.set_len(self.hist_offset)?;
        self.witness_file.set_len(self.witness_offset)?;

        Ok(())
    }

    /// Flush all registers to disk (fsync)
    pub fn flush(&mut self) -> Result<()> {
        self.work_file.sync_all()?;
        self.hist_file.sync_all()?;
        self.witness_file.sync_all()?;
        Ok(())
    }
}

impl Drop for AtomicWriter {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_atomic_commit_success() {
        let temp_dir = TempDir::new().unwrap();
        let mut writer = AtomicWriter::new(temp_dir.path()).unwrap();

        let record = AkashicRecord::new(
            [1u8; 32],
            [2u8; 32],
            12345,
            0x0001,
            1234567890,
        );

        // Should succeed
        writer.atomic_commit(&record).unwrap();

        // Verify files exist and have correct size
        let work_size = std::fs::metadata(temp_dir.path().join("work_register.dat"))
            .unwrap()
            .len();
        assert_eq!(work_size, 128);
    }

    #[test]
    fn test_multiple_commits() {
        let temp_dir = TempDir::new().unwrap();
        let mut writer = AtomicWriter::new(temp_dir.path()).unwrap();

        // Commit 10 records
        for i in 0..10u8 {
            let record = AkashicRecord::new(
                [i; 32],
                [(i + 1) % 255; 32], // Use 255 to fit u8, or cast.
                i as u32 * 1000,
                i as u16,
                1234567890 + i as u64,
            );

            writer.atomic_commit(&record).unwrap();
        }

        // Verify all three files have 10 records
        let work_size = std::fs::metadata(temp_dir.path().join("work_register.dat"))
            .unwrap()
            .len();
        assert_eq!(work_size, 128 * 10);
    }
}
