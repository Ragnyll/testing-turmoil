use std::net::TcpListener;

fn handle_client<T: std::io::Read>(mut stream: T) -> Result<(), std::io::Error> {
    let mut bfr = [0_u8; 64];
    loop {
        match stream.read(&mut bfr) {
            Ok(size) => {
                let msg = &bfr[0..size];
                println!("Received message: {msg:?}");
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

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr)?;

    println!("Listening on address: {}", addr);
    loop {
        for stream in listener.incoming() {
            handle_client(stream?).unwrap();
        }
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

        let mock_stream = StubTcpStream {};
        assert_eq!(handle_client(mock_stream).unwrap(), ());
    }
}
