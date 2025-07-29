/*
* to run this project it is important to use a bare metal environment
*
* to install the target triple:
* rustup target add thumbv7em-none-eabihf
*
* to run using this taret triple:
* cargo build --target thumbv7em-none-eabihf
*/

#![no_std]
#![no_main]
use core::panic::PanicInfo;

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}