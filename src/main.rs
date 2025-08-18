#![no_std] // don't link the std library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)] // use nightly feature to define own test framework
#![test_runner(nacho_os::test_runner)] // use the test_runner crate to run the tests
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
//use nacho_os::println;
// use x86_64::instructions::port::Port;

mod vga_buffer;
mod serial;

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

// this function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nacho_os::test_panic_handler(info);
}