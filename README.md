# FractureOS

A modern Unix-like OS kernel written in Rust.

## Build

```bash
rustup component add rust-src llvm-tools-preview
cargo install bootimage
cargo run --release
```

## Features

- Memory management (paging, heap allocator)
- Interrupt handling (IDT, PIC, exceptions)
- Async task executor
- VGA text mode, serial port, keyboard driver
- System call interface
- Process management framework
- Virtual file system
