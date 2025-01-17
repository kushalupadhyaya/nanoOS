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
            if val != 42 {
                loop {} // In a real system, handle error accordingly.
            }
        }
        _ => {}
    }
    loop {}
}
