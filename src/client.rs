use tokio::net::TcpStream;

pub struct Client {
    socket: TcpStream,
}

impl Client {
    pub fn new(socket: TcpStream) -> Self {
        Client { socket }
    }

    pub async fn handle(&self) {}
}
