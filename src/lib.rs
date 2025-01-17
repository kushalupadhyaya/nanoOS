#![no_std]

// Provide a single panic handler for the entire crate.
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Declare the submodules. Ensure each referenced module file exists.
pub mod scheduler;
pub mod interrupts;
pub mod ipc;
pub mod power;
pub mod arch;
pub mod drivers;

/// Kernel initialization function.
pub fn init() {
    // Perform early hardware initialization.
    arch::init_arch();
    interrupts::init_interrupts();
    scheduler::init_scheduler();
    power::init_power();
}
