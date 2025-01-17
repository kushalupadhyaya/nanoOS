#![no_std]
#![no_main]

use nanokernel_rs::init;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// The entry point for the nanokernel.
#[no_mangle]
pub extern "C" fn main() -> ! {
    init();
    loop {
        // Nanokernel idle loop
    }
}
