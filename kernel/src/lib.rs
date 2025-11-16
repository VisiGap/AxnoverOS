#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod ipc;
pub mod memory;
pub mod process;
pub mod serial;
pub mod shm;
pub mod signal;
pub mod syscall;
pub mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("FractureOS Kernel v0.1.0");
    serial_println!("Initializing...");

    // Initialize GDT
    serial_println!("[INIT] Setting up GDT...");
    gdt::init();

    // Initialize IDT
    serial_println!("[INIT] Setting up IDT...");
    interrupts::init_idt();

    // Initialize PIC
    serial_println!("[INIT] Initializing PIC...");
    unsafe { interrupts::PICS.lock().initialize() };

    // Enable interrupts
    x86_64::instructions::interrupts::enable();
    serial_println!("[INIT] Interrupts enabled");

    // Initialize process management
    serial_println!("[INIT] Initializing process manager...");
    process::init();

    // Initialize IPC
    serial_println!("[INIT] Initializing IPC...");
    ipc::init();

    // Initialize signals
    serial_println!("[INIT] Initializing signal system...");
    signal::init();

    // Initialize shared memory
    serial_println!("[INIT] Initializing shared memory...");
    shm::init();

    // Initialize system calls
    serial_println!("[INIT] Initializing system calls...");
    syscall::init();

    // Print system info
    serial_println!("\n=== FractureOS System Info ===");
    serial_println!("Kernel: v0.1.0");
    serial_println!("Architecture: x86_64");
    serial_println!("Process Manager: Ready");
    serial_println!("Heap Size: {} KB", allocator::HEAP_SIZE / 1024);
    serial_println!("==============================\n");

    serial_println!("Kernel initialized successfully");
    serial_println!("Entering idle loop...");

    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("KERNEL PANIC: {}", info);
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
