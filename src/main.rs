#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

mod vga_buffer;
mod allocator;
mod memory;
mod interrupts;
mod gdt;
mod task;
mod serial;
mod syscall;
mod process;
mod fs;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    use x86_64::VirtAddr;
    
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║           Welcome to FractureOS v0.1.0               ║");
    println!("║     A Modern Rust-based Unix-like Operating System   ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();
    
    // Initialize kernel subsystems
    println!("[KERNEL] Initializing GDT...");
    gdt::init();
    
    println!("[KERNEL] Initializing IDT...");
    interrupts::init_idt();
    
    println!("[KERNEL] Initializing PIC...");
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    
    println!("[KERNEL] Initializing memory management...");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_regions)
    };
    
    println!("[KERNEL] Initializing heap allocator...");
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
    
    println!("[KERNEL] Initializing task executor...");
    task::init();
    
    println!();
    println!("[OK] FractureOS kernel initialized successfully!");
    println!("[INFO] System ready for operation");
    println!();
    
    #[cfg(test)]
    test_main();
    
    // Start the async executor
    println!("[KERNEL] Starting async task executor...");
    let mut executor = task::executor::Executor::new();
    executor.spawn(task::Task::new(example_task()));
    executor.spawn(task::Task::new(task::keyboard::print_keypresses()));
    executor.run();
}

async fn example_task() {
    println!("[TASK] Async task system operational");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!();
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║                  KERNEL PANIC!                        ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[FAILED]");
    println!("Error: {}", info);
    hlt_loop();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
