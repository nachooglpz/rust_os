#![no_std] // don't link the std library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

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

    loop {}
}