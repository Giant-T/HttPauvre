use request::request::Request;
use response::response::Response;
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::TcpListener,
};

mod request;
mod response;
mod status;

#[tokio::main]
async fn main() {
    let host = "localhost:8080";
    let listener = TcpListener::bind(host).await.unwrap();
    println!("Server running on {}", host);

    open::that(format!("http://{}", host)).unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            println!("Request from : {}", addr.ip().to_string());
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            let req = Request::from_tcp_reader(&mut reader).await;
            let res = Response::from_request(req).await;

            writer.write_all(&res.as_bytes()).await.unwrap();
        });
    }
}
