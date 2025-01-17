// src/ipc.rs
#![no_std]

/// A simple message type for inter-process communication.
#[derive(Copy, Clone)]
pub enum Message {
    Empty,
    Data(u32),
}

/// Send a message (stub implementation).
pub fn send_message(msg: Message) {
    // In a real system, enqueue the message for the target process.
    let _ = msg;
}

/// Receive a message (stub implementation).
pub fn receive_message() -> Message {
    // Normally would retrieve a queued message.
    Message::Empty
}
