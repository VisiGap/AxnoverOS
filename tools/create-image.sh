#!/bin/bash
# Create bootable disk image for FractureOS

set -e

BUILD_DIR="build"
IMAGE="$BUILD_DIR/fractureos.img"

echo "Creating FractureOS disk image..."

# Create 10MB disk image
dd if=/dev/zero of="$IMAGE" bs=1M count=10

# Write bootloader
dd if="$BUILD_DIR/boot.bin" of="$IMAGE" conv=notrunc bs=512 count=1

# Write stage 2
dd if="$BUILD_DIR/stage2.bin" of="$IMAGE" conv=notrunc bs=512 seek=1 count=4

# Write kernel (starting at sector 6)
if [ -f "$BUILD_DIR/kernel.bin" ]; then
    dd if="$BUILD_DIR/kernel.bin" of="$IMAGE" conv=notrunc bs=512 seek=5
fi

echo "Disk image created: $IMAGE"
echo "Boot with: qemu-system-x86_64 -drive format=raw,file=$IMAGE"
