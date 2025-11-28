//! Dragon Tools: Core Data Types
//!
//! CRITICAL: All memory is explicitly sanctified (zeroed)

use serde::{Deserialize, Serialize};

/// The Akashic Record: 78 bytes of data + 50 bytes of sacred padding
///
/// SAFETY GUARANTEE: All 128 bytes are explicitly initialized to prevent
/// undefined behavior and information leakage into the permanent ledger.
#[repr(C, align(128))]
#[derive(Debug, Clone, Copy)]
pub struct AkashicRecord {
    /// BLAKE3 hash of current state (32 bytes)
    pub h_completion: [u8; 32],

    /// Link to previous record in chain (32 bytes)
    pub h_previous: [u8; 32],

    /// Unix timestamp in nanoseconds (8 bytes)
    /// Reordered to ensure 8-byte alignment without implicit padding
    pub t_commit: u64,

    /// Phase lock angle × 100 (0-35999) (4 bytes)
    pub t_pl: u32,

    /// Hieroglyph operation ID (2 bytes)
    pub h_id: u16,

    /// SACRED PADDING: Explicitly zeroed to sanctify the record
    /// CRITICAL: Must be exactly 50 bytes to reach 128-byte alignment
    /// 78 bytes data + 50 bytes padding = 128 bytes total
    _padding: [u8; 50],
}

impl AkashicRecord {
    /// Total size in bytes (cache-line aligned)
    pub const SIZE: usize = 128;

    /// Data size (excluding padding)
    pub const DATA_SIZE: usize = 78;

    /// SAFE constructor: Ensures ALL memory is sanctified
    ///
    /// # Safety
    /// This method guarantees that all 128 bytes (including padding)
    /// are explicitly initialized, preventing undefined behavior.
    pub fn new(
        h_completion: [u8; 32],
        h_previous: [u8; 32],
        t_pl: u32,
        h_id: u16,
        t_commit: u64,
    ) -> Self {
        Self {
            h_completion,
            h_previous,
            t_commit, // Reordered
            t_pl,
            h_id,
            _padding: [0u8; 50],  // 🔥 SANCTIFIED: All padding zeroed
        }
    }

    /// Returns the record as a byte slice (SAFE: all bytes initialized)
    ///
    /// # Safety
    /// This is safe because `new()` guarantees all bytes are initialized.
    /// The slice includes the full 128 bytes for cache-line alignment.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                Self::SIZE
            )
        }
    }

    /// Verifies that all padding bytes are sanctified (zeroed)
    ///
    /// Use this in debug builds to catch any corruption.
    pub fn is_sanctified(&self) -> bool {
        self._padding.iter().all(|&byte| byte == 0)
    }

    /// Create a record from raw bytes (with validation)
    ///
    /// # Safety
    /// This method validates that the input is exactly 128 bytes
    /// and that all padding bytes are zero (sanctified).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != Self::SIZE {
            return Err("Invalid record size");
        }

        // SAFETY: We verified the length above
        let record = unsafe {
            std::ptr::read(bytes.as_ptr() as *const Self)
        };

        // Verify sanctification
        if !record.is_sanctified() {
            return Err("Padding bytes not sanctified (non-zero)");
        }

        Ok(record)
    }
}

/// Phase angle in degrees (0-359.99)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhaseAngle(f64);

impl PhaseAngle {
    pub fn new(degrees: f64) -> Self {
        Self(degrees.rem_euclid(360.0))
    }

    pub fn degrees(&self) -> f64 {
        self.0
    }

    pub fn to_u32(&self) -> u32 {
        (self.0 * 100.0).round() as u32
    }

    pub fn from_u32(value: u32) -> Self {
        Self::new((value as f64) / 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_size() {
        assert_eq!(std::mem::size_of::<AkashicRecord>(), 128);
        assert_eq!(AkashicRecord::SIZE, 128);
    }

    #[test]
    fn test_record_sanctification() {
        let record = AkashicRecord::new(
            [1u8; 32],
            [2u8; 32],
            12345,
            0x0001,
            1234567890,
        );

        assert!(record.is_sanctified(), "Padding must be zeroed");

        let bytes = record.as_bytes();
        assert_eq!(bytes.len(), 128);

        // Verify padding region is all zeros
        let padding_start = 78;
        assert!(bytes[padding_start..].iter().all(|&b| b == 0));
    }

    #[test]
    fn test_phase_angle() {
        let phase = PhaseAngle::new(270.5);
        assert_eq!(phase.degrees(), 270.5);
        assert_eq!(phase.to_u32(), 27050);

        let restored = PhaseAngle::from_u32(27050);
        assert_eq!(restored.degrees(), 270.5);
    }
}
