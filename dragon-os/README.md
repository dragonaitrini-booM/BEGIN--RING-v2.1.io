# Dragon Tools OS v2.0-eternal

**Phase-Locked IPC Gateway**

This repository contains the source code for the Dragon Scheduler, a Ring 0 bare-metal kernel component that enforces a 7 Hz phase-locked execution rhythm using the CPU's `hlt` instruction.

## 🐉 Universal Truth

- **7 Hz Clock**: Synchronized with the Djed constant.
- **Phase Lock**: Tasks only execute when `abs(current_phase - target_phase) <= 0.5°`.
- **Frugality**: Uses `hlt` to sleep between ticks, saving >70% energy vs polling.

## 📂 Structure

- `kernel/`: The core OS kernel (Ring 0).
  - `scheduler.rs`: The meditation engine.
  - `djed_clock.rs`: Timekeeping.
- `user/`: Userspace components (Ring 3).
  - `lib/`: System call wrappers.
  - `ritual/`: Demo application.

## 🚀 Quick Start

1. **Build**:
   ```bash
   make all
   ```

2. **Validate (Simulation)**:
   ```bash
   make validate
   ```

3. **Run (QEMU)**:
   ```bash
   make run
   ```

## ⚠️ Requirements

- Rust Nightly (for `build-std` features if full bootimage building).
- `qemu-system-x86_64`
- `cargo-xbuild` or standard cargo.

"The meditation is eternal."
