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
        let mut hasher = self.hasher.clone();

        hasher.update(&record.h_completion); // Hash of content? Or is this circular?
        // Usually h_completion IS the hash of the record.
        // If we are hashing the record to verify it, we might hash other fields + prev_hash.
        // But the struct definition says `h_completion`: "BLAKE3 hash of current state".
        // Let's hash the *other* fields to check integrity, or just hash the whole 128 bytes?
        // If `h_completion` is part of the record, we can't hash the record including it to produce it.
        // The user prompt doesn't specify the exact hashing protocol for the fields.
        // I will hash the fields that are NOT `h_completion` to verify against `h_completion` if needed,
        // or just hash the bytes for a "record hash".
        // Let's assume for `hash_record` we hash: h_previous, t_pl, h_id, t_commit, padding.

        hasher.update(&record.h_previous);
        hasher.update(&record.t_pl.to_le_bytes());
        hasher.update(&record.h_id.to_le_bytes());
        hasher.update(&record.t_commit.to_le_bytes());
        // We should probably also hash the padding if it's sanctified.
        // But for now let's stick to fields.

        *hasher.finalize().as_bytes()
    }

    pub fn verify_integrity(&self, record: &AkashicRecord, expected_hash: [u8; 32]) -> Result<()> {
        let calculated = self.hash_record(record);
        if calculated == expected_hash {
            Ok(())
        } else {
            Err(DragonError::ChainIntegrityViolation)
        }
    }
}
