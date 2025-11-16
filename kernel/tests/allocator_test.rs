#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use fracture_kernel::{serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn test_heap_allocation() {
    serial_print!("test_heap_allocation... ");
    let heap_value = Box::new(42);
    assert_eq!(*heap_value, 42);
    serial_println!("[ok]");
}

#[test_case]
fn test_vec_allocation() {
    serial_print!("test_vec_allocation... ");
    let mut vec = Vec::new();
    for i in 0..100 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 100);
    serial_println!("[ok]");
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    loop {}
}
