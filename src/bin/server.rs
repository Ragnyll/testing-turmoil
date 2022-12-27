use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
