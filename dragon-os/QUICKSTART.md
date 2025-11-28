# Quick Start Guide

## Prerequisites

- Rust (nightly recommended for OS dev)
- QEMU (for running the kernel)
- Make

## Build & Run

1. **Navigate to the directory**:
   ```bash
   cd dragon-os
   ```

2. **Build the kernel**:
   ```bash
   make all
   ```

3. **Run the simulation**:
   ```bash
   make run
   ```
   *Note: This requires QEMU installed and in your PATH.*

4. **Verify output**:
   Look for the "Awakened at phase..." logs in the QEMU output.

## Troubleshooting

- **Linker errors**: Ensure `rust-lld` is installed (`rustup component add llvm-tools-preview` usually helps).
- **Target errors**: Ensure you are using the provided `x86_64-dragon-os.json` target file.
