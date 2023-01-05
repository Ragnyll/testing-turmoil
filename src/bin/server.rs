use std::net::TcpListener;

pub struct ConnectionHandler {
    listener: Option<TcpListener>,
}

impl ConnectionHandler {
    fn new(listener: TcpListener) -> Self {
        Self {
            listener: Some(listener),
        }
    }
}

pub trait HandleClients<T: std::io::Read + std::io::Write>: HandleClient<T> {
    fn handle_clients(&self) -> std::io::Result<()>;
}

pub trait HandleClient<T: std::io::Read + std::io::Write> {
    fn handle_client(stream: T) -> Result<(), std::io::Error>;
}

// this trait should be integration tested
impl<T: std::io::Read + std::io::Write> HandleClients<T> for ConnectionHandler {
    fn handle_clients(&self) -> std::io::Result<()> {
        match &self.listener {
            Some(l) => {
                for stream in l.incoming() {
                    match stream {
                        Ok(stream) => {
                            println!("New connection: {}", stream.peer_addr().unwrap());
                            std::thread::spawn(move || {
                                // connection succeeded
                                ConnectionHandler::handle_client(stream)
                            });
                        }
                        Err(e) => {
                            // connection failed
                            println!("Error: {}", e);
                        }
                    }
                }
            }
            // This will only be hit in test. the new constructor requires that it be defined.
            // Only a test will construct a
            None => {
                // define an error type type for this. the std::io::Result should not actually be
                // used here
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Listener is not defined.",
                ));
            }
        }

        Ok(())
    }
}

// This trait is unit tested
impl<T: std::io::Read + std::io::Write> HandleClient<T> for ConnectionHandler {
    fn handle_client(mut stream: T) -> Result<(), std::io::Error> {
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
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8000";
    let listener = TcpListener::bind(addr)?;
    let connection_handler = ConnectionHandler::new(listener);

    println!("Listening on address: {}", addr);
    // I keep the loop because i want the user to know that this will indefinately run
    loop {
        <ConnectionHandler as HandleClients<std::net::TcpStream>>::handle_clients(
            &connection_handler,
        )?;
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

        let stream_stub = StubTcpStream {};
        assert_eq!(ConnectionHandler::handle_client(stream_stub).unwrap(), ());
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

        let stream_stub = StubTcpStream {};
        assert_eq!(ConnectionHandler::handle_client(stream_stub).unwrap(), ());
    }
}
