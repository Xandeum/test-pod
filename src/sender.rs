use std::{thread, time::Duration};
use zmq;

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::PUB).unwrap();

    socket.bind("ipc:///tmp/socket").unwrap();

    let mut counter = 0;

    println!("Sender started. sending messages...");

    loop {
        let message = format!("Message {}", counter);
        socket.send(&message, 0).unwrap();
        println!("Sent: {}", message);

        counter += 1;
        thread::sleep(Duration::from_secs(3)); 
    }
}
