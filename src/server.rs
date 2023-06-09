use std::{net::Ipv4Addr, time::Duration};

use log::info;
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::{tcp::ReadHalf, TcpListener},
};

use crate::{
    config::Config, request::request::Request, response::response::Response, status::HttpStatusCode,
};

pub struct Server {
    host: String,
    port: u32,
    timeout: u64,
}

impl Server {
    pub fn new(host: Ipv4Addr, port: u32) -> Self {
        return Server {
            host: host.to_string(),
            port,
            timeout: 5,
        };
    }

    pub fn from_config() -> Self {
        let config: Config = toml::from_str(include_str!("../config.toml")).unwrap();

        return Server {
            host: config.server.host,
            port: config.server.port,
            timeout: config.server.timeout,
        };
    }

    ///
    /// Genere la réponse à partir de la requête du client.
    ///
    async fn generate_response(reader: BufReader<ReadHalf<'_>>) -> Response {
        let req = Request::parse_request(reader).await;
        let result = Response::from_request(req).await;

        if let Err(error) = result {
            return Response::generate_error_response(HttpStatusCode::from(error));
        }

        return result.unwrap();
    }

    ///
    /// Démarre le serveur sur l'hôte et le port définis lors de sa création.
    ///
    pub async fn start(self) -> ! {
        let formatted_host = format!("{}:{}", self.host, self.port);

        let listener = TcpListener::bind(&formatted_host).await.unwrap();

        info!("server running on {}", formatted_host);

        loop {
            let (mut socket, addr) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                info!("request from : {}", addr.ip().to_string());
                let (reader, mut writer) = socket.split();
                let reader = BufReader::new(reader);

                tokio::select! {
                    res = Self::generate_response(reader) => {
                        _ = writer.write_all(&res.as_bytes()).await;
                    }
                    _ = tokio::time::sleep(Duration::from_secs(self.timeout)) => {
                        let res = Response::generate_error_response(HttpStatusCode::RequestTimeout);
                        _ = writer.write_all(&res.as_bytes()).await;
                    }
                }
            });
        }
    }
}
