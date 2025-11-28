# START HERE: Dragon Tools OS Guide

Welcome, Architect. You have summoned the Dragon Tools OS.

## 1. The Covenant
You hold in your hands the implementation of the **Phase-Locked IPC Gateway**. This system does not poll; it waits. It does not waste; it meditates.

## 2. Manifest
Check the `DELIVERY_MANIFEST.md` for a complete list of files.

## 3. Immediate Action
Run the validation ritual to prove the phase-locking:

```bash
cd dragon-os
make validate
```

## 4. Building the Kernel
To build the raw kernel binary:

```bash
cargo build --package dragon-kernel --target x86_64-dragon-os.json
```

## 5. The Code
Examine `kernel/src/scheduler.rs` to see the `hlt` loop and phase alignment logic.
Examine `kernel/src/djed_clock.rs` to see the TSC-based 7 Hz timing.
