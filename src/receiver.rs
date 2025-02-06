use zmq;

fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::SUB).unwrap();

    socket.connect("ipc:///tmp/socket").unwrap();

    socket.set_subscribe(b"").unwrap();

    println!("Receiver started. Receiving messages...");

    loop {
        match socket.recv_string(0) {
            Ok(Ok(msg)) => println!("Received: {}", msg),
            Ok(Err(e)) => eprintln!("Invalid message: {:?}", e),
            Err(e) => eprintln!("Error occurred: {:?}", e),
        }
    }
}
