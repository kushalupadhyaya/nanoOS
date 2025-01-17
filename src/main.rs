#![no_std]
#![no_main]

// Import the init function from the library crate.
use nanokernel_rs::init;

#[no_mangle]
pub extern "C" fn main() -> ! {
    init();
    loop {
        // Nanokernel idle loop.
    }
}
