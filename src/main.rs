#![no_std] // don't link the std library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)] // use nightly feature to define own test framework
#![test_runner(crate::test_runner)] // use the test_runner crate to run the tests
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

mod vga_buffer;

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // the linker looks for a function called '_start'
    // so this function is the entry point
    
    println!("Hello world{}", "!");
    // panic!("Some panic message");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

