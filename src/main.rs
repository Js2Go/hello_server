// use std::thread;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        
        pool.execute(|| {
            handle_connection(stream);
        });

        // println!("Connection established!");
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // if buffer.starts_with(get) {
    //     let contents = fs::read_to_string("hello.html").unwrap();
    
    //     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // } else {
    // }
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // println!("Request: {}\n\n", String::from_utf8_lossy(&buffer[..]));
}
