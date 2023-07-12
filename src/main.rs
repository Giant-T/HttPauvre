use crate::server::Server;

mod config;
mod request;
mod response;
mod server;
mod status;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let server = Server::from_config(); 

    server.start().await;
}
