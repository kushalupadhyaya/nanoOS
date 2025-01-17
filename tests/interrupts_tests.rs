// tests/interrupts_tests.rs
#![no_std]
#![no_main]

extern crate nanokernel_rs;
use nanokernel_rs::interrupts;

#[no_mangle]
pub extern "C" fn main() -> ! {
    interrupts::init_interrupts();
    // Since irq_handler is a stub, we simply call it.
    unsafe { interrupts::irq_handler() };
    loop {}
}
