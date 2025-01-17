#![no_std]

pub mod scheduler;
pub mod interrupts;
pub mod ipc;
pub mod power;
pub mod arch;
pub mod drivers;

/// Kernel initialization function.
pub fn init() {
    // Perform early hardware init.
    arch::init_arch();
    interrupts::init_interrupts();
    scheduler::init_scheduler();
    power::init_power();
}
