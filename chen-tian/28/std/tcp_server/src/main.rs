use std::{io::{Read, Write}, net::TcpListener, thread};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9710").unwrap();
    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Accepted a new connection: {}", addr);
        thread::spawn(move || {
            let mut buf = [0u8; 12];
            stream.read_exact(&mut buf).unwrap();
            println!("data: {:?}", String::from_utf8_lossy(&buf));
            stream.write_all(b"glad to meet you!").unwrap();
        });
    }
}
