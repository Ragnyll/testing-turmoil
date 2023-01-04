use std::net::TcpListener;

fn handle_client<T: std::io::Read + std::io::Write>(mut stream: T) -> Result<(), std::io::Error> {
    let mut bfr = [0_u8; 64];
    loop {
        match stream.read(&mut bfr) {
            Ok(size) => {
                let msg = &mut bfr[0..size];
                stream.write(msg)?;
                // return if the funciton is under test
                #[cfg(test)]
                {
                    return Ok(());
                }
            }
            Err(err) => {
                println!("received error: {err}");
                // return if the function is under test
                #[cfg(test)]
                {
                    return Err(err);
                }
            }
        }
    }
}

/// Handle the incoming connections by assigning an indivdual worker to it.
fn handle_incoming_connections(listener: &TcpListener) -> std::io::Result<()> {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                std::thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr)?;

    println!("Listening on address: {}", addr);
    // I keep the loop becaue i want the user to know that this will indefinately run
    loop {
        handle_incoming_connections(&listener)?;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn test_handle_client_err() {
        struct StubTcpStream {}
        impl std::io::Read for StubTcpStream {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Make sure we can hit that error",
                ))
            }
        }
        impl std::io::Write for StubTcpStream {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Ok(0_usize)
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let mock_stream = StubTcpStream {};
        assert_eq!(handle_client(mock_stream).unwrap(), ());
    }

    #[test]
    fn test_handle_client_ok() {
        struct StubTcpStream {}
        impl std::io::Read for StubTcpStream {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Ok(0_usize)
            }
        }
        impl std::io::Write for StubTcpStream {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Ok(0_usize)
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let mock_stream = StubTcpStream {};
        assert_eq!(handle_client(mock_stream).unwrap(), ());
    }
}
