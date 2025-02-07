use std::{thread::sleep, time::Duration};

use prost::Message;
use solana_sdk::{pubkey::Pubkey, signer::Signer};
use zmq;
mod types {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
}

use types::Request;

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::SUB).unwrap();

    socket.connect("ipc:///tmp/socket").unwrap();

    socket.set_subscribe(b"").unwrap();

    println!("Receiver started. Receiving messages...");

    loop {
        match socket.recv_bytes(zmq::DONTWAIT) {
            Ok(msg) => {
                let req = Request::decode(&msg[..]).unwrap();
                let pk = Pubkey::from(<[u8; 32]>::try_from(&req.pubkey[..]).unwrap());
                println!("{:?}", pk);
                println!("Received: {:?}", req)
            }
            Err(zmq::Error::EAGAIN) => {
                println!("No message Received, sleeping for 2 Seconds");
                sleep(Duration::from_secs(2));
            }

            Err(e) => eprintln!("Error occurred: {:?}", e),
        }
    }
}
