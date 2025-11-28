//! IPC Stub
//! Since we don't have NVMe hardware, this mocks the atomic commit logic.

use crate::types::AkashicRecord;

pub fn commit(record: &AkashicRecord) {
    // In a real system, this would write to NVMe via io_uring
    // Here we just log the event
    log::info!("IPC: Committing record ID {} at phase {}", record.h_id, record.t_pl);
}
