// tests/scheduler_tests.rs
#![no_std]
#![no_main]

extern crate nanokernel_rs;
use nanokernel_rs::scheduler;

#[no_mangle]
pub extern "C" fn main() -> ! {
    scheduler::init_scheduler();
    let task = scheduler::current_task();
    assert!(task == 1);
    loop {}
}
