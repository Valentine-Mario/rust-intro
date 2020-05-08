use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    let server = TcpListener::bind("127.0.0.1:3005").unwrap();
    for stream in server.incoming() {
        let stream = stream.unwrap();
        let pool = ThreadPool::new(3);

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    //declear buffer of 4096 byte
    let mut buffer = [0; 4096];

    //read data from stream and store it in a buffer
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_code_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "web_server.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", "sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    //convert byte from buffer and store it in string
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{} {}", status_code_line, contents);

    //convert the string response to byte
    stream.write(response.as_bytes()).unwrap();
    //flush waits until all the byte are written to connaection
    stream.flush().unwrap();
}
