// src/scheduler.rs
#![no_std]

static mut CURRENT_TASK: u32 = 0;

/// Initialize the scheduler.
pub fn init_scheduler() {
    // Basic scheduler initialization.
    unsafe {
        CURRENT_TASK = 1;
    }
}

/// Minimal context switch (stub implementation).
pub fn context_switch() {
    // In a complete implementation, switch task context.
    unsafe {
        CURRENT_TASK = (CURRENT_TASK + 1) % 10; // Simulate round-robin.
    }
}

/// Return the ID of the current task.
pub fn current_task() -> u32 {
    unsafe { CURRENT_TASK }
}
