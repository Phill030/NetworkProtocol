use std::net::{SocketAddr, TcpStream};

pub struct Client {
    pub stream: TcpStream,
    pub addr: SocketAddr,
}

impl Client {
    #[must_use]
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        Self { stream, addr }
    }
}
