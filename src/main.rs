/*
* to run this project it is important to use a bare metal environment
* the guide recommends the ARM Cortex-M (embedded) with hardware floating point (hf).
*
* to install the target triple:
* 'rustup target add thumbv7em-none-eabihf'
*
* the /.cargo/config.toml file is already configured to use the ARM Cortex-M
* so it is safe to just run the project using:
* 'cargo build'
*
* or to cargo build using this target triple directly:
* 'cargo build --target thumbv7em-none-eabihf'
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