use std::net::TcpStream;
use std::io::{ Read, Write };
fn main() {
    let mut stream = TcpStream::connect("localhost:3001").unwrap();
    stream.write("hello buff".as_bytes()).unwrap();

    let mut buffer = [0; 10];
    stream.read(&mut buffer).unwrap();

    println!(
        "Got response from server: {:?}",
        std::str::from_utf8(&buffer).unwrap()
    );
}
