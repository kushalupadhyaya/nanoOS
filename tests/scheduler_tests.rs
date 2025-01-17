#![no_std]
#![no_main]

extern crate nanokernel_rs;
use nanokernel_rs::scheduler;

#[no_mangle]
pub extern "C" fn main() -> ! {
    scheduler::init_scheduler();
    let task = scheduler::current_task();
    // Simple check: after init_scheduler, the current task should be 1.
    if task != 1 {
        // In an embedded environment without std, you might trigger an LED or similar.
        loop {}
    }
    loop {}
}
