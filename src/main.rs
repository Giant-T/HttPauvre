use request::Request;
use response::send_response;
use tokio::{
    io::BufReader,
    net::{
        tcp::{ReadHalf, WriteHalf},
        TcpListener,
    },
};

mod request;
mod response;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            loop {
                if handle_request(&mut reader, &mut writer).await {
                    break;
                }
            }
        });
    }
}

///
/// Fonction qui envoie la bonne réponse au client.
///
/// Retourne vrai si une réponse à été envoyée au client.
///
async fn handle_request(reader: &mut BufReader<ReadHalf<'_>>, writer: &mut WriteHalf<'_>) -> bool {
    let req = Request::from_tcp_reader(reader).await;

    if let None = req {
        return false;
    }

    let req = req.unwrap();

    send_response(req, writer).await;

    return true;
}
