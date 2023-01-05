use std::{net::TcpStream, io::prelude::*};

fn main() -> Result<(), std::io::Error> {
    match TcpStream::connect("127.0.0.1:8000") {
        Ok(mut stream) => {
            let msg = b"lol what it do?";

            stream.write(msg)?;
        }
        Err(err) => {
            eprintln!("unable to connect");
            return Err(err);
        }
    }

    Ok(())
}
