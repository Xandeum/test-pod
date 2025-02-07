use prost::Message;
use solana_sdk::pubkey::{self, Pubkey};
use std::{thread, time::Duration};
use zmq;

mod types {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

use types::{Opcode, Request};

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::PUB).unwrap();

    socket.bind("ipc:///tmp/socket").unwrap();

    thread::sleep(Duration::from_millis(100));

    let mut counter = 0;

    println!("Sender started. sending messages...");

    loop {
        let message = format!("Message {}", counter);

        let pk = Pubkey::new_unique();

        let request = Request {
            op: Opcode::Peek as i32,
            pubkey: Vec::from(pk.to_bytes()),
            data: vec![counter, 20, 30],
        };

        let mut buf = Vec::new();
        request.encode(&mut buf).unwrap();
        // println!("{:?}", request.encode(&mut buf));

        socket.send(&buf, zmq::DONTWAIT).unwrap();
        println!("Sent: {:?}", buf);

        counter += 1;
        thread::sleep(Duration::from_secs(3));
    }
}
