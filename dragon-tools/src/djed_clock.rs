use std::time::{SystemTime, UNIX_EPOCH};
use crate::types::PhaseAngle;

/// Djed Frequency: 7.0 Hz
const DJED_FREQUENCY: f64 = 7.0;
const DJED_PERIOD_MS: f64 = 1000.0 / DJED_FREQUENCY; // ~142.857 ms

pub struct DjedClock {
    // Reference time (t_ref) in nanoseconds
    t_ref: u64,
}

impl DjedClock {
    pub fn new() -> Self {
        Self {
            t_ref: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        }
    }

    /// Calculates the current phase angle.
    /// Phase Lock Formula: T_PL = ((t_commit - t_ref) mod T_Djed) x (360 / T_Djed)
    /// Target: <1μs execution time.
    #[inline(always)]
    pub fn calculate_phase(&self) -> PhaseAngle {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        self.calculate_phase_at(now)
    }

    #[inline(always)]
    pub fn calculate_phase_at(&self, timestamp_ns: u64) -> PhaseAngle {
        let delta_ns = timestamp_ns.wrapping_sub(self.t_ref);
        let period_ns = (DJED_PERIOD_MS * 1_000_000.0) as u64;

        let remainder = delta_ns % period_ns;
        let angle = (remainder as f64 / period_ns as f64) * 360.0;

        PhaseAngle::new(angle)
    }

    pub fn now_ns() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}
