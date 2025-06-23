mod client;
mod resp;
mod server;
mod store;

use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = server::Server::new();
    server.run().await
}
