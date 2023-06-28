use std::{env, time::Duration};

use log::{error, info};
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::{tcp::ReadHalf, TcpListener},
    time::timeout,
};

use crate::{request::request::Request, response::response::Response, status::HttpStatusCode};

pub struct Server {
    host: Box<str>,
    port: Box<str>,
    pub timeout_s: u64,
}

impl Server {
    pub fn new(host: &str, port: &str) -> Self {
        return Server {
            host: Box::from(host),
            port: Box::from(port),
            timeout_s: env::var("TIMEOUT_S")
                .unwrap_or("5".to_string())
                .parse()
                .unwrap_or(5),
        };
    }

    ///
    /// Genere la réponse à partir de la requête du client.
    ///
    async fn generate_response(reader: BufReader<ReadHalf<'_>>, res: &mut Response) {
        let req = Request::from_tcp_reader(reader).await;
        *res = Response::from_request(req).await;
    }

    ///
    /// Démarre le serveur sur l'hôte et le port définis lors de sa création.
    ///
    pub async fn start(self) -> ! {
        let formatted_host = format!("{}:{}", self.host, self.port);

        let listener = TcpListener::bind(&formatted_host).await.unwrap();

        info!("server running on {}", formatted_host);

        open::that(format!("http://{}", formatted_host)).unwrap();

        loop {
            let (mut socket, addr) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                info!("request from : {}", addr.ip().to_string());
                let (reader, mut writer) = socket.split();
                let reader = BufReader::new(reader);

                let mut res: Response = Response::default();

                if let Err(_) = timeout(
                    Duration::from_secs(self.timeout_s),
                    Self::generate_response(reader, &mut res),
                )
                .await
                {
                    error!("connection timed out");
                    res = Response::default();
                    res.status = HttpStatusCode::RequestTimeout as u32;

                    res.add_header("Content-Type", "text/html");
                    res.add_header("Connection", "close");

                    res.content = "".as_bytes().to_vec();
                }

                writer.write_all(&res.as_bytes()).await.unwrap();
            });
        }
    }
}
