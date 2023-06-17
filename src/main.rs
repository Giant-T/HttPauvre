use std::time::Duration;

use log::{error, info};
use request::request::Request;
use response::response::Response;
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::{tcp::ReadHalf, TcpListener},
    time::timeout,
};

use crate::status::HttpStatusCode;

mod request;
mod response;
mod status;

const TIMEOUT_S: u64 = 10;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    env_logger::init();

    let host = "localhost:8080";
    let listener = TcpListener::bind(host).await.unwrap();
    info!("server running on {}", host);

    open::that(format!("http://{}", host)).unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            info!("request from : {}", addr.ip().to_string());
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);

            let mut res: Response = Response::default();

            if let Err(_) = timeout(Duration::from_secs(TIMEOUT_S), async {
                res = future(&mut reader).await;
            })
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

async fn future(reader: &mut BufReader<ReadHalf<'_>>) -> Response {
    let req = Request::from_tcp_reader(reader).await;
    return Response::from_request(req).await;
}
