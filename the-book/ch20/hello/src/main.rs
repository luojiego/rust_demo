use std::{io::prelude::*, thread, time::Duration};
use std::net::TcpListener;
use std::net::TcpStream;
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        // println!("Connection established: {:?}", stream);
        // handle_connection(stream);
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

use std::fs;

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
// 
//     let contents = fs::read_to_string("hello.html").unwrap();
//     // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
//     // let response = "HTTP/1.1 200 OK\r\n\r\n";
// 
//     let get = b"GET / HTTP/1.1\r\n";
//     if buffer.starts_with(get) {
//         let response = format!(
//             "HTTP/1.1 200 OK\r\nContent-Length: {} \r\n\r\n{}",
//             contents.len(),
//             contents
//         );
// 
//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     } else {
//         let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
//         let contents = fs::read_to_string("404.html").unwrap();
// 
//         let response = format!("{}{}", status_line, contents);
// 
//         stream.write(response.as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }
// }

// 优化后的代码
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

