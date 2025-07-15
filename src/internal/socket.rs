use std::io::Read;
use std::net::TcpListener;

#[derive(Debug, Clone)]
pub enum SocketError {
    BindError(String),
    AcceptError(String),
    ReadError(String),
}

impl std::fmt::Display for SocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Socket Error")
    }
}

pub trait Socket {
    fn bind(&mut self, host: &str, port: u16) -> Result<(), SocketError>;
    fn accept(&mut self) -> Result<Box<dyn Connection>, SocketError>;
}

pub struct TcpSocket {
    listener: Option<TcpListener>,
}

impl TcpSocket {
    pub fn new() -> Self {
        TcpSocket { listener: None }
    }
}

impl Socket for TcpSocket {
    fn bind(&mut self, host: &str, port: u16) -> Result<(), SocketError> {
        let addr = format!("{}:{}", host, port);

        match TcpListener::bind(&addr) {
            Ok(listener) => {
                self.listener = Some(listener);
                Ok(())
            }
            Err(_) => Err(SocketError::BindError(format!(
                "Failed to bind to {}",
                addr
            ))),
        }
    }

    fn accept(&mut self) -> Result<Box<dyn Connection>, SocketError> {
        if let Some(listener) = &self.listener {
            match listener.accept() {
                Ok((stream, addr)) => {
                    let connection = TcpConnection {
                        stream,
                        address: addr,
                    };
                    Ok(Box::new(connection))
                }
                Err(_) => Err(SocketError::AcceptError(
                    "Failed to accept connection".to_string(),
                )),
            }
        } else {
            Err(SocketError::BindError("Socket not bound".to_string()))
        }
    }
}

pub struct TcpConnection {
    stream: std::net::TcpStream,
    address: std::net::SocketAddr,
}

pub trait Connection {
    fn address(&self) -> &std::net::SocketAddr;
    fn receive(&mut self, buffer_size: usize) -> Result<Vec<u8>, SocketError>;
}

impl Connection for TcpConnection {
    fn address(&self) -> &std::net::SocketAddr {
        &self.address
    }

    fn receive(&mut self, buffer_size: usize) -> Result<Vec<u8>, SocketError> {
        let mut buffer = vec![0; buffer_size];

        match self.stream.read(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(_) => Err(SocketError::ReadError(
                "Failed to read from connection".to_string(),
            )),
        }
    }
}

pub fn socket() -> Box<dyn Socket> {
    Box::new(TcpSocket::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_creation() {
        let mut socket = socket();

        assert!(socket.bind("localhost", 12000).is_ok());
    }
}
