use io_uring::{IoUring, opcode, types};
use std::os::unix::io::AsRawFd;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use crate::types::AkashicRecord;
use crate::error::{Result, DragonError};

/// Handles atomic writes using io_uring to three registers (Work, History, Witness).
/// Target: ~15μs write latency (total for batch).
pub struct AtomicWriter {
    ring: IoUring,
    work_file: File,
    hist_file: File,
    wit_file: File,
}

impl AtomicWriter {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base = base_path.as_ref();
        let work_path = base.with_file_name(format!("{}_work.log", base.file_name().unwrap().to_string_lossy()));
        let hist_path = base.with_file_name(format!("{}_hist.log", base.file_name().unwrap().to_string_lossy()));
        let wit_path = base.with_file_name(format!("{}_wit.log", base.file_name().unwrap().to_string_lossy()));

        let open_opts = |p: &PathBuf| -> Result<File> {
            Ok(OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                // .custom_flags(libc::O_DIRECT) // Commented out: O_DIRECT requires block-aligned buffers (usually 512/4096).
                                                 // Our 128-byte record is aligned in memory but not size-aligned to blocks.
                                                 // Implementing full O_DIRECT support would require aligned 4k buffers.
                .open(p)?)
        };

        let work_file = open_opts(&work_path)?;
        let hist_file = open_opts(&hist_path)?;
        let wit_file = open_opts(&wit_path)?;

        // Setup io_uring with adequate queue depth
        let ring = IoUring::new(128)?;

        Ok(Self {
            ring,
            work_file,
            hist_file,
            wit_file,
        })
    }

    /// Submits a batched, linked write for the Akashic Record to all three registers.
    /// Uses IOSQE_IO_LINK to ensure atomicity (all or nothing chain execution).
    pub fn write_record(&mut self, record: &AkashicRecord) -> Result<()> {
        let buf = record.as_bytes();
        let ptr = buf.as_ptr();
        let len = buf.len() as u32;

        let fds = [
            self.work_file.as_raw_fd(),
            self.hist_file.as_raw_fd(),
            self.wit_file.as_raw_fd(),
        ];

        // We will submit 3 entries.
        // We link them so if one fails, the chain breaks.
        // Note: io_uring guarantees submission order.

        let mut sq = self.ring.submission();

        // We need to ensure the ring has space
        // If not enough space, we drop sq and submit to clear/process, then re-acquire.
        if sq.capacity() - sq.len() < 3 {
             drop(sq);
             self.ring.submit()?;
             sq = self.ring.submission();
        }

        // Entry 1: Work Register
        let entry1 = opcode::Write::new(types::Fd(fds[0]), ptr, len)
            .offset(u64::MAX) // Append
            .build()
            .flags(io_uring::squeue::Flags::IO_LINK) // Link to next
            .user_data(record.id);

        // Entry 2: History Register
        let entry2 = opcode::Write::new(types::Fd(fds[1]), ptr, len)
            .offset(u64::MAX)
            .build()
            .flags(io_uring::squeue::Flags::IO_LINK) // Link to next
            .user_data(record.id);

        // Entry 3: Witness Register (Last one, no link)
        let entry3 = opcode::Write::new(types::Fd(fds[2]), ptr, len)
            .offset(u64::MAX)
            .build()
            .user_data(record.id);

        unsafe {
            sq.push(&entry1).map_err(|_| DragonError::RingSubmissionError)?;
            sq.push(&entry2).map_err(|_| DragonError::RingSubmissionError)?;
            sq.push(&entry3).map_err(|_| DragonError::RingSubmissionError)?;
        }

        // Drop sq so we can call submit_and_wait
        drop(sq);

        self.ring.submit_and_wait(3)?;

        // Check completions
        // We expect 3 completions.
        let mut cqs = self.ring.completion();

        for _ in 0..3 {
            let cqe = cqs.next().ok_or(DragonError::Io(std::io::Error::last_os_error()))?;
            if cqe.result() < 0 {
                return Err(DragonError::Io(std::io::Error::from_raw_os_error(-cqe.result())));
            }
        }

        Ok(())
    }
}
