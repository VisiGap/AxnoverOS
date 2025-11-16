# FractureOS Development Roadmap

## Phase 1: Core Kernel (Current)
- [x] Basic bootloader
- [x] GDT and IDT setup
- [x] Interrupt handling
- [x] VGA text mode output
- [x] Serial port communication
- [x] Physical memory manager ✅
- [x] Virtual memory manager ✅
- [x] Heap allocator ✅
- [x] Basic process management ✅

## Phase 2: System Calls & IPC ✅
- [x] System call interface ✅
- [x] Process creation (fork/exec) ✅
- [x] Message-passing IPC ✅
- [x] Shared memory ✅
- [x] Signals ✅

## Phase 3: Userspace Foundation
- [ ] Init process
- [ ] Basic shell
- [ ] VFS layer
- [ ] Device manager
- [ ] Standard library (libfracture)

## Phase 4: Device Drivers
- [ ] Keyboard driver
- [ ] ATA/AHCI disk driver
- [ ] Network card driver (e1000)
- [ ] USB support
- [ ] Graphics driver (basic framebuffer)

## Phase 5: Filesystem
- [ ] VFS implementation
- [ ] ext2 filesystem
- [ ] tmpfs
- [ ] procfs
- [ ] devfs

## Phase 6: Networking
- [ ] Network stack (TCP/IP)
- [ ] Socket API
- [ ] DHCP client
- [ ] DNS resolver
- [ ] Basic HTTP client

## Phase 7: Advanced Features
- [ ] SMP support
- [ ] ACPI support
- [ ] Power management
- [ ] Dynamic linking
- [ ] ELF loader improvements

## Phase 8: Userspace Tools
- [ ] Core utilities (ls, cat, cp, mv, etc.)
- [ ] Text editor
- [ ] Package manager
- [ ] Build system
- [ ] Debugger

## Phase 9: GUI (Future)
- [ ] Window manager
- [ ] GUI toolkit
- [ ] Terminal emulator
- [ ] File manager
- [ ] Web browser (basic)

## Long-term Goals
- Self-hosting capability
- POSIX compatibility layer
- Container support
- Security hardening
- Performance optimization
