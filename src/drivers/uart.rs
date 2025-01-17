// src/drivers/uart.rs
#![no_std]

/// Initialize UART for serial communication.
pub fn init_uart() {
    // Configure UART registers.
}

/// Send a byte over UART.
pub fn uart_send(byte: u8) {
    let _ = byte; // Stub: actually transmit.
}

/// Receive a byte over UART (stub).
pub fn uart_receive() -> u8 {
    0
}
