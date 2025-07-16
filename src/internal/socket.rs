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

#[derive(Debug, Clone, PartialEq)]
pub enum Socket {
    Tcp(TcpSocket),
}

impl Socket {
    fn as_socketable(&mut self) -> &mut dyn Socketable {
        match self {
            Socket::Tcp(tcp_socket) => tcp_socket,
        }
    }

    pub fn bind(&mut self, host: &str, port: u16) -> Result<(), SocketError> {
        self.as_socketable().bind(host, port)
    }
}

pub trait Socketable {
    fn bind(&mut self, host: &str, port: u16) -> Result<(), SocketError>;
    fn accept(&mut self) -> Result<Connection, SocketError>;
}

#[derive(Debug)]
pub struct TcpSocket {
    pub listener: Option<TcpListener>,
}

impl PartialEq for TcpSocket {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Clone for TcpSocket {
    fn clone(&self) -> Self {
        TcpSocket { listener: None }
    }
}

impl Socketable for TcpSocket {
    fn bind(&mut self, host: &str, port: u16) -> Result<(), SocketError> {
        let address = format!("{}:{}", host, port);

        match TcpListener::bind(&address) {
            Ok(listener) => {
                self.listener = Some(listener);

                Ok(())
            }
            Err(e) => Err(SocketError::BindError(e.to_string())),
        }
    }

    fn accept(&mut self) -> Result<Connection, SocketError> {
        if let Some(listener) = &self.listener {
            match listener.accept() {
                Ok((stream, _)) => Ok(Connection::Tcp(TcpConnection { stream })),
                Err(e) => Err(SocketError::AcceptError(e.to_string())),
            }
        } else {
            Err(SocketError::AcceptError(
                "Listener is not initialized".to_string(),
            ))
        }
    }
}

pub enum Connection {
    Tcp(TcpConnection),
}

pub trait Connectionable {
    fn read(&mut self, buffer: usize) -> Result<Vec<u8>, SocketError>;
}

pub struct TcpConnection {
    pub stream: std::net::TcpStream,
}

impl Connectionable for TcpConnection {
    fn read(&mut self, buffer_size: usize) -> Result<Vec<u8>, SocketError> {
        let mut buffer = vec![0; buffer_size];

        match self.stream.read(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(e) => Err(SocketError::ReadError(e.to_string())),
        }
    }
}
