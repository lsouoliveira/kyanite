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

#[derive(Debug)]
pub enum Socket {
    Tcp(TcpSocket),
}

impl Socket {
    pub fn as_socketable(&mut self) -> &mut dyn Socketable {
        match self {
            Socket::Tcp(tcp_socket) => tcp_socket,
        }
    }

    pub fn bind(&mut self, host: &str, port: u16) -> Result<(), SocketError> {
        self.as_socketable().bind(host, port)
    }

    pub fn accept(&mut self) -> Result<Connection, SocketError> {
        self.as_socketable().accept()
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

impl Socketable for TcpSocket {
    fn bind(&mut self, host: &str, port: u16) -> Result<(), SocketError> {
        let parsed_host = if host == "localhost" {
            "127.0.0.1"
        } else {
            host
        };

        let address = format!("{}:{}", parsed_host, port);

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
                Ok((stream, _)) => {
                    println!("Accepted connection from {}", stream.peer_addr().unwrap());
                    Ok(Connection::Tcp(TcpConnection { stream }))
                }
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

impl Connection {
    pub fn as_connectionable(&mut self) -> &mut dyn Connectionable {
        match self {
            Connection::Tcp(tcp_connection) => tcp_connection,
        }
    }

    pub fn read(&mut self, buffer_size: usize) -> Result<Vec<u8>, SocketError> {
        self.as_connectionable().read(buffer_size)
    }
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

pub fn create_socket() -> Socket {
    Socket::Tcp(TcpSocket { listener: None })
}
