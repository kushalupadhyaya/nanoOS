# Nanokernel-RS

A minimal nanokernel written in Rust targeting resource-constrained devices.

## Overview

This project provides a basic framework for a nanokernel that includes:
- A minimalist scheduler.
- Basic interrupt handling.
- Simple inter-process communication (IPC).
- Power management stubs.
- Architecture-specific and driver modules.

## Building

Ensure that your target is configured in `.cargo/config.toml` and run:

```bash
cargo build --release
