// src/arch/mod.rs

pub mod arch_specific;

/// Architecture initialization.
pub fn init_arch() {
    arch_specific::setup_arch();
}
