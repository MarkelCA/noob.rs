// Same example that the tokyo one but with standard rust itself.
//
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        match stream.read(&mut buf) {
            Ok(n) if n == 0 => {
                // Connection closed by client
                break;
            }
            Ok(n) => {
                // Echo the received data back to the client
                if let Err(e) = stream.write_all(&buf[0..n]) {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                break;
            }
        }
    }
}

pub fn example() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("failed to accept client connection; err = {:?}", e);
            }
        }
    }

    Ok(())
}
