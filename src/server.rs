use crate::client;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub struct Server {
    socket_addr: SocketAddr,
}

impl Server {
    pub fn new() -> Server {
        Server {
            socket_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)),
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let listener = tokio::net::TcpListener::bind(self.socket_addr).await?;

        loop {
            let (socket, addr) = listener.accept().await?;

            tokio::spawn(async move {
                let client = client::Client::new(socket);
                client.handle().await;
            });
        }
    }
}
