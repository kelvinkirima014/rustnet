use std::{net::TcpListener, io::{Read, Write}};

fn main() -> std::io::Result<()> {
    
    let listener = TcpListener::bind("127.0.0.1:3001")?;

    println!("Server running on port 3001");

    for connection in listener.incoming() {
        let mut connection = connection.unwrap();
        println!("Connection established");

        let mut buffer = [0; 1024];
        connection.read(&mut buffer).unwrap();
        connection.write(&mut buffer).unwrap();
    }
    
    Ok(())

}
