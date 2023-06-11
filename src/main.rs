use request::request::Request;
use response::response::send_response;
use tokio::{
    io::BufReader,
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
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            loop  {
                let req = Request::from_tcp_reader(&mut reader).await;
                send_response(req, &mut writer).await;
                break;
            }
        });
    }
}

