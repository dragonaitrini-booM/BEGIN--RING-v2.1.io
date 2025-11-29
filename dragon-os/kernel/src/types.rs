//! Shared types for Dragon Tools OS

// Phase is stored as hundredths of a degree (0..36000)
// This avoids floating point in the kernel while maintaining 0.01° precision
pub type DjedPhaseAngle = u16;

#[repr(C, align(128))]
#[derive(Clone, Copy)]
pub struct AkashicRecord {
    pub h_completion: [u8; 32],
    pub h_previous: [u8; 32],
    pub t_pl: u32,
    pub h_id: u16,
    pub t_commit: u64,
    // Critical: Explicit padding to 128 bytes, must be zeroed
    pub _padding: [u8; 50],
}

impl AkashicRecord {
    pub const fn new(h_completion: [u8; 32], h_previous: [u8; 32], t_pl: u32, h_id: u16, t_commit: u64) -> Self {
        Self {
            h_completion,
            h_previous,
            t_pl,
            h_id,
            t_commit,
            _padding: [0; 50],
        }
    }
}

// System State constants
pub const STATE_MEDITATING: u8 = 0;
pub const STATE_MANIFESTING: u8 = 1;
pub const STATE_PANIC: u8 = 255;
