// src/drivers/uart.rs

/// Initialize UART for serial communication.
pub fn init_uart() {
    // Configure UART registers (stub implementation).
}

/// Send a byte over UART.
pub fn uart_send(byte: u8) {
    let _ = byte; // Stub: actually transmit the byte.
}

/// Receive a byte over UART (stub).
pub fn uart_receive() -> u8 {
    0
}
