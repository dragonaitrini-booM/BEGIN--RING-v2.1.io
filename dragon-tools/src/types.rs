use std::fmt;
use std::mem::MaybeUninit;

/// Represents the phase angle in the Djed clock cycle (0.0 to 360.0 degrees).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PhaseAngle(pub f64);

impl PhaseAngle {
    pub fn new(angle: f64) -> Self {
        Self(angle % 360.0)
    }

    pub fn is_coherent(&self, tolerance: f64) -> bool {
        // Coherent if close to 0 (or 360) within tolerance
        let diff = self.0.abs().min((360.0 - self.0).abs());
        diff <= tolerance
    }
}

impl fmt::Display for PhaseAngle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}°", self.0)
    }
}

/// The atomic unit of storage in the Akashic Ledger.
/// Total size of data is 78 bytes.
/// Aligned to 128 bytes (cache line) for performance.
#[repr(C, align(128))]
#[derive(Debug, Clone, Copy)]
pub struct AkashicRecord {
    pub id: u64,                  // 8 bytes
    pub timestamp: u64,           // 8 bytes (nanoseconds)
    pub phase: f64,               // 8 bytes
    pub prev_hash: [u8; 32],      // 32 bytes
    pub payload: [u8; 22],        // 22 bytes
    // Total used: 8 + 8 + 8 + 32 + 22 = 78 bytes
    // Padding to 128 bytes is handled by align(128)
}

impl Default for AkashicRecord {
    fn default() -> Self {
        // Safe because we are initializing with 0 which is valid for all fields (u64, f64, arrays)
        // and we want padding to be zeroed.
        unsafe {
            let record = MaybeUninit::<AkashicRecord>::zeroed();
            record.assume_init()
        }
    }
}

impl AkashicRecord {
    pub fn new(id: u64, timestamp: u64, phase: PhaseAngle, prev_hash: [u8; 32], payload_data: &[u8]) -> Self {
        let mut record = Self::default(); // Zero-initialized including padding

        record.id = id;
        record.timestamp = timestamp;
        record.phase = phase.0;
        record.prev_hash = prev_hash;

        let len = payload_data.len().min(22);
        record.payload[..len].copy_from_slice(&payload_data[..len]);

        record
    }

    pub fn as_bytes(&self) -> &[u8] {
        // Safe cast to slice of bytes including padding.
        // Since we initialized with MaybeUninit::zeroed(), padding is guaranteed to be 0.
        unsafe {
            std::slice::from_raw_parts(
                self as *const _ as *const u8,
                std::mem::size_of::<Self>()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_akashic_record_alignment() {
        assert_eq!(mem::align_of::<AkashicRecord>(), 128);
        assert_eq!(mem::size_of::<AkashicRecord>(), 128);
    }

    #[test]
    fn test_padding_is_zero() {
        let record = AkashicRecord::new(1, 2, PhaseAngle(3.0), [1; 32], &[0xFF; 22]);
        let bytes = record.as_bytes();

        // Data ends at byte 78.
        // 78 to 128 should be zero.
        for i in 78..128 {
            assert_eq!(bytes[i], 0, "Padding byte at index {} is not zero", i);
        }
    }
}
