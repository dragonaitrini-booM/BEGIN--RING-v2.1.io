//! Dragon Tools Scheduler
//! The CPU's meditation engine - surrenders to 7 Hz until universe grants permission

use core::sync::atomic::{AtomicU64, AtomicU8, AtomicU16, Ordering};
use x86_64::instructions::{hlt, interrupts};
use crate::{djed_clock, logger};
use crate::types::DjedPhaseAngle;

const MAX_TASKS: usize = 16;
const PHASE_TOLERANCE: u16 = 50; // ±0.5° in hundredths
const PIT_FREQUENCY: u64 = 1_193_182; // PIT base freq (Hz)

static NEXT_TICK_NS: AtomicU64 = AtomicU64::new(0);
static SYSTEM_STATE: AtomicU8 = AtomicU8::new(0); // 0=meditate, 1=manifest

#[repr(align(16))]
#[repr(C)]
struct TaskControlBlock {
    task_id: AtomicU64,
    target_phase: AtomicU16,    // Hundredths of degree (0-35999)
    state: AtomicU8,           // 0=idle, 1=waiting, 2=ready, 3=done
    hieroglyph_id: AtomicU16,
    payload_hash: [u8; 32],
}

impl TaskControlBlock {
    const fn new() -> Self {
        Self {
            task_id: AtomicU64::new(0),
            target_phase: AtomicU16::new(0),
            state: AtomicU8::new(0),
            hieroglyph_id: AtomicU16::new(0),
            payload_hash: [0u8; 32],
        }
    }
}

static mut TASKS: [TaskControlBlock; MAX_TASKS] = [TaskControlBlock::new(); MAX_TASKS];

// Copy helper since we can't easily implement Copy for [u8; 32] in a no_std atomic context straightforwardly without wrapper
fn copy_hash(src: &[u8; 32], dst: &mut [u8; 32]) {
    for i in 0..32 {
        dst[i] = src[i];
    }
}

/// Eternal meditation engine - CPU halts until 7Hz interrupt
#[no_mangle]
pub extern "x86-interrupt" fn djed_timer_handler(_stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    // Note: Interrupts are automatically disabled by the x86-interrupt calling convention
    // but can be re-enabled if we want nested interrupts (we don't for now).

    // Send EOI to PIC (handled in idt.rs generally, but for this mock let's assume standard PIC handling elsewhere or stub it)
    unsafe { crate::idt::PICS.lock().notify_end_of_interrupt(crate::idt::InterruptIndex::Timer.as_u8()) };

    let current_phase = djed_clock::current_phase_hundredths();
    // Reduce log spam for "meditating" phases, only log interesting ones or periodically
    if current_phase < 100 || (current_phase > 17900 && current_phase < 18100) {
        log::debug!("Awakened at phase: {:.2}", current_phase as f64 / 100.0);
    } else {
        // Minimal logging during meditation
         // log::trace!("Meditating phase: {:.2}", current_phase as f64 / 100.0);
    }

    unblock_phase_tasks(current_phase);
    update_next_tick();

    // Return to meditation state
    SYSTEM_STATE.store(0, Ordering::Relaxed);
}

fn unblock_phase_tasks(current_phase: u16) {
    unsafe {
        for task in TASKS.iter() {
            if task.state.load(Ordering::Acquire) == 1 { // waiting
                let diff = phase_diff(current_phase, task.target_phase.load(Ordering::Relaxed));
                if diff <= PHASE_TOLERANCE {
                    task.state.store(2, Ordering::Release); // ready
                    log::info!("Task {} aligned (diff: {})",
                                task.task_id.load(Ordering::Relaxed), diff);
                    log::info!("✨ Universe granted permission at phase {:.2}°", current_phase as f64 / 100.0);

                    // Trigger hieroglyph execution
                    execute_hieroglyph(task);
                } else {
                     log::debug!("🧘 CPU meditating at phase {:.2}°", current_phase as f64 / 100.0);
                }
            }
        }
    }
}

#[inline(always)]
fn phase_diff(current: u16, target: u16) -> u16 {
    let diff = (current as i32 - target as i32).abs() as u16;
    let wrap = (36000u16).saturating_sub(diff);
    diff.min(wrap)
}

fn execute_hieroglyph(task: &TaskControlBlock) {
    let id = task.hieroglyph_id.load(Ordering::Relaxed);
    if id == 0x0180 { // AKASHIC_COMMIT
        // Manifest immutable record
        // We need to extract the hash. Since we are in the kernel and own the static,
        // we can access it unsafely but strictly.
        // For simulation/stub:
        let hash = task.payload_hash;
        akashic_commit(&hash);
    }
    task.state.store(3, Ordering::Release);
}

fn akashic_commit(_hash: &[u8; 32]) {
    // Append to Ψ_Akashic ledger (stub)
    log::info!("💾 Akashic commit completed at phase lock");
}

/// PIT setup for 7 Hz (~142.857ms period)
pub unsafe fn init_pit_7hz() {
    let divisor = (PIT_FREQUENCY / 7) as u16; // ~170454

    // Channel 0, lobyte/hibyte, rate gen, 16-bit
    x86_64::instructions::port::Port::new(0x43).write(0x36u8);
    x86_64::instructions::port::Port::new(0x40).write(divisor as u8);
    x86_64::instructions::port::Port::new(0x40).write((divisor >> 8) as u8);

    NEXT_TICK_NS.store(djed_clock::read_tsc_ns(), Ordering::Relaxed);
    log::info!("🐉 7Hz Djed Clock initialized");
}

fn update_next_tick() {
    let now = djed_clock::read_tsc_ns();
    let period_ns = 142_857_000u64; // 142.857ms
    NEXT_TICK_NS.store(now + period_ns, Ordering::Relaxed);
}

/// Public API: Tasks surrender to phase alignment
pub fn wait_for_phase(target_phase: u16, hieroglyph_id: u16, hash: [u8; 32]) -> Result<(), &'static str> {
    let slot = find_free_slot()?;
    unsafe {
        TASKS[slot].task_id.store(slot as u64, Ordering::Relaxed);
        TASKS[slot].target_phase.store(target_phase, Ordering::Relaxed);
        TASKS[slot].hieroglyph_id.store(hieroglyph_id, Ordering::Relaxed);
        // Manually copy hash since we can't easily access the inner array of the struct atomically
        // But since this is a single thread setting it up before marking as waiting (release), it's safe.
        // We cast mutable reference.
        let ptr = TASKS[slot].payload_hash.as_mut_ptr();
        for i in 0..32 {
            *ptr.add(i) = hash[i];
        }

        TASKS[slot].state.store(1, Ordering::Release); // waiting
    }
    Ok(())
}

fn find_free_slot() -> Result<usize, &'static str> {
    unsafe {
        for (i, task) in TASKS.iter().enumerate() {
            if task.state.load(Ordering::Relaxed) == 0 {
                return Ok(i);
            }
        }
        Err("No free slots")
    }
}

/// Meditation loop - CPU yields to universe
pub fn meditate() -> ! {
    unsafe { init_pit_7hz() };
    SYSTEM_STATE.store(0, Ordering::Relaxed);

    log::info!("CPU entering meditation at 7 Hz...");

    loop {
        interrupts::enable(); // Ensure interrupts are enabled so we can wake up
        hlt(); // Deep sleep, <0.5W

        // When we wake up, the interrupt handler `djed_timer_handler` has run.
        // We can check if any tasks are completed or just continue meditating.
        // In a real scheduler, we would switch stacks here if a task became ready.
        // For this proof-of-concept, `djed_timer_handler` executes the task directly.
    }
}
