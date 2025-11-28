#![no_std]
#![no_main]

mod scheduler;
mod djed_clock;
mod logger;
mod types;
mod idt;
mod gdt;
mod ipc;

use core::panic::PanicInfo;
use x86_64::instructions::{interrupts, port};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    logger::init();
    logger::info!("🐉 Dragon Tools OS v2.0-eternal");

    // Initialize hardware
    gdt::init();
    idt::init();

    // Initialize Djed Clock
    unsafe {
        djed_clock::init(0); // 0 = calibrate
    }

    logger::info!("Phase-Locked IPC Gateway Active");
    logger::info!("CPU entering meditation at 7 Hz...");

    // Example: Schedule Akashic commit at 180°
    let hash = [0u8; 32]; // Stub hash
    if let Err(e) = scheduler::wait_for_phase(18000, 0x0180, hash) {
        logger::error!("Failed to schedule task: {}", e);
    }

    // Enter eternal meditation
    scheduler::meditate();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logger::error!("{}", info);
    loop { x86_64::instructions::hlt() }
}
