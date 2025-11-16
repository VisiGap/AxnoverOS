# UEFI Bootloader

This directory contains the UEFI bootloader for FractureOS.

## Requirements

To build the UEFI bootloader, you need:

- GNU-EFI library and headers
- GCC compiler
- GNU ld linker

### Installation

**Ubuntu/Debian:**
```bash
sudo apt install gnu-efi
```

**Arch Linux:**
```bash
sudo pacman -S gnu-efi
```

**Fedora:**
```bash
sudo dnf install gnu-efi gnu-efi-devel
```

## Building

```bash
make -C boot/uefi
```

This will create `build/BOOTX64.EFI`.

## Testing

```bash
make run-uefi
```

## Note

The UEFI bootloader is optional. FractureOS can boot using the BIOS bootloader (boot.asm + stage2.asm) which is the default.

For development, the BIOS bootloader is recommended as it has fewer dependencies.
