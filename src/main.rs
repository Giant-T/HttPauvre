use crate::server::Server;

mod request;
mod response;
mod server;
mod status;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let server = Server::new("192.168.2.23", "8080");

    server.start().await;
}
