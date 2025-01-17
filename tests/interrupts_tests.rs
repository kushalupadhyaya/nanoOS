#![no_std]
#![no_main]

extern crate nanokernel_rs;
use nanokernel_rs::interrupts;

#[no_mangle]
pub extern "C" fn main() -> ! {
    interrupts::init_interrupts();
    // Call the interrupt handler (stub); in a real test, you may simulate an interrupt.
    unsafe { interrupts::irq_handler() };
    loop {}
}
