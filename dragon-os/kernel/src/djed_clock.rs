//! Djed Clock - 7 Hz Time Source
//! Implements the phase calculation and TSC timing

use x86_64::instructions::time::rdtsc;
use crate::types::DjedPhaseAngle;

// Constants for 7 Hz
// 1 second = 1,000,000,000 ns
// 7 Hz period = 142,857,142 ns
pub const DJED_PERIOD_NS: u64 = 142_857_142;

// Assume a fixed TSC frequency for the prototype (e.g., 2.0 GHz)
// In a real system, this would be calibrated at boot
pub const TSC_FREQUENCY_HZ: u64 = 2_000_000_000;

pub unsafe fn init(_tsc_freq: u64) {
    // Calibration logic stub
}

pub fn read_tsc() -> u64 {
    unsafe { rdtsc() }
}

pub fn read_tsc_ns() -> u64 {
    let tsc = read_tsc();
    // Convert cycles to nanoseconds: (tsc * 1_000_000_000) / TSC_FREQ
    // Use u128 to prevent overflow during multiplication
    ((tsc as u128 * 1_000_000_000) / (TSC_FREQUENCY_HZ as u128)) as u64
}

/// Calculate current phase in hundredths of a degree (0-36000)
pub fn current_phase_hundredths() -> DjedPhaseAngle {
    let ns = read_tsc_ns();
    let phase_ns = ns % DJED_PERIOD_NS;

    // Convert to angle: (phase_ns / period_ns) * 36000
    // Result is 0..36000
    ((phase_ns as u128 * 36000) / (DJED_PERIOD_NS as u128)) as u16
}

pub fn tick_to_nanoseconds(ticks: u64) -> u64 {
    ticks * DJED_PERIOD_NS
}

pub fn read_tick_count() -> u64 {
    let ns = read_tsc_ns();
    ns / DJED_PERIOD_NS
}
