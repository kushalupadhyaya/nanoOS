/// A simple message type for inter-module communication.
#[derive(Copy, Clone)]
pub enum Message {
    Empty,
    Data(u32),
}

/// Send a message (stub implementation).
pub fn send_message(msg: Message) {
    let _ = msg; // Stub: Enqueue the message in a real implementation.
}

/// Receive a message (stub implementation).
pub fn receive_message() -> Message {
    Message::Empty // Stub: Return a queued message in a real implementation.
}
