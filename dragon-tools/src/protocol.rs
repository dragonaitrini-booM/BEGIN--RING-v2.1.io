use blake3::Hasher;
use crate::types::AkashicRecord;
use crate::error::{Result, DragonError};

pub struct ProtocolStack {
    hasher: Hasher,
}

impl ProtocolStack {
    pub fn new() -> Self {
        Self {
            hasher: Hasher::new(),
        }
    }

    /// Computes the hash of a record, chaining it with the previous hash.
    /// Target: ~3μs
    pub fn hash_record(&self, record: &AkashicRecord) -> [u8; 32] {
        let mut hasher = self.hasher.clone(); // In actual chaining, we might update state.
                                              // But here, let's assume we just hash the record bytes.

        // For strict chaining, we might want: hash(prev_hash || record_data)
        // The record already contains prev_hash.

        // Using blake3::hash for one-shot or hasher update.
        // Assuming we want to hash the significant bytes (first 78 bytes)
        // or the whole 128 bytes if we want to include padding (safer to just do used bytes).

        // Let's hash the structure fields specifically to be deterministic across architectures
        // (though repr(C) helps).

        hasher.update(&record.id.to_le_bytes());
        hasher.update(&record.timestamp.to_le_bytes());
        hasher.update(&record.phase.to_le_bytes());
        hasher.update(&record.prev_hash);
        hasher.update(&record.payload);

        *hasher.finalize().as_bytes()
    }

    pub fn verify_integrity(&self, record: &AkashicRecord, expected_hash: [u8; 32]) -> Result<()> {
        let calculated = self.hash_record(record);
        if calculated == expected_hash {
            Ok(())
        } else {
            Err(DragonError::CryptoError)
        }
    }
}
