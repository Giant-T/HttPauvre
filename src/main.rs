use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{ReadHalf, WriteHalf},
        TcpListener,
    },
};

enum FileFormat {
    Png,
    Html,
    NotSupported,
}

const DIR: &str = "www";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            loop {
                if send_response(&mut reader, &mut writer).await {
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
async fn send_response(reader: &mut BufReader<ReadHalf<'_>>, writer: &mut WriteHalf<'_>) -> bool {
    let mut buf = String::new();
    let bytes = reader.read_line(&mut buf).await.unwrap();

    if bytes != 0 && buf.contains("GET") {
        let args = buf.split(" ").collect::<Vec<&str>>();
        println!("{}", args[1]);

        let file_name: &str;
        if args[1].len() == 1 {
            file_name = "index.html";
        } else {
            file_name = &args[1][1..];
        }

        let mut res = String::new();
        let file_format = match file_name.split(".").last().unwrap() {
            "png" => FileFormat::Png,
            "html" => FileFormat::Html,
            _ => FileFormat::NotSupported,
        };

        match (
            fs::File::open(format!("{}/{}", DIR, file_name)).await,
            file_format,
        ) {
            (Ok(mut file), FileFormat::Html) => {
                let mut content = String::new();
                file.read_to_string(&mut content).await.unwrap();

                res.push_str("HTTP/1.1 200 OK\n");
                res.push_str("Content-Type: text/html\n\n");
                res.push_str(&content);
            }
            (Ok(mut file), FileFormat::Png) => {
                // Envoie de fichier png
                let mut response = Vec::<u8>::new();

                let file_length = file.metadata().await.unwrap().len();
                let mut content = Vec::<u8>::with_capacity(file_length as usize);
                let _ = file.read_to_end(&mut content).await.unwrap();

                response = [response, "HTTP/1.1 200 OK\n".as_bytes().to_vec()].concat();
                response = [response, "Content-Type: image/png\n".as_bytes().to_vec()].concat();
                response = [
                    response,
                    format!("Content-Length: {}\n\n", file_length)
                        .as_bytes()
                        .to_vec(),
                ]
                .concat();

                response = [response, content].concat();

                writer.write_all(&response).await.unwrap();
                return true;
            }
            (_, _) => {
                res.push_str("HTTP/1.1 404 NOT FOUND\n");
                res.push_str("Content-Type: text/html\n\n");
                res.push_str("<h1> 404 - Page not found </h1>");
            }
        };

        writer.write_all(res.as_bytes()).await.unwrap();
        return true;
    }

    return false;
}
