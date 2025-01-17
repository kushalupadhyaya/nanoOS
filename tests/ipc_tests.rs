// tests/ipc_tests.rs
#![no_std]
#![no_main]

extern crate nanokernel_rs;
use nanokernel_rs::ipc::{send_message, receive_message, Message};

#[no_mangle]
pub extern "C" fn main() -> ! {
    send_message(Message::Data(42));
    let msg = receive_message();
    match msg {
        Message::Data(val) => {
            assert!(val == 42);  // This assert may fail with stub, but demonstrates intent.
        }
        _ => {}
    }
    loop {}
}
