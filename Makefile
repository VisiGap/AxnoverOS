.PHONY: all build run clean test help

# Default target
all: build

# Build the kernel
build:
	@echo "Building FractureOS kernel..."
	cargo build --release

# Create bootable image
bootimage:
	@echo "Creating bootable disk image..."
	cargo bootimage --release

# Run in QEMU
run:
	@echo "Starting FractureOS in QEMU..."
	cargo run --release

# Run with KVM acceleration (Linux only)
run-kvm:
	@echo "Starting FractureOS in QEMU with KVM..."
	cargo run --release -- -enable-kvm

# Run tests
test:
	@echo "Running kernel tests..."
	cargo test

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Check code without building
check:
	@echo "Checking code..."
	cargo check

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Run clippy linter
clippy:
	@echo "Running clippy..."
	cargo clippy

# Show help
help:
	@echo "FractureOS Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  make build      - Build the kernel"
	@echo "  make bootimage  - Create bootable disk image"
	@echo "  make run        - Run in QEMU"
	@echo "  make run-kvm    - Run in QEMU with KVM (Linux)"
	@echo "  make test       - Run tests"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make check      - Check code without building"
	@echo "  make fmt        - Format code"
	@echo "  make clippy     - Run clippy linter"
	@echo "  make help       - Show this help message"
