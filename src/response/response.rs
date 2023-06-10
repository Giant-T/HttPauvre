use std::str::FromStr;

use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::WriteHalf,
};

use crate::{request::request::Request, status::HttpStatusCode};

use super::file::FileType;

pub const DIR: &str = "www";

///
/// Envoie la réponse au client http.
///
pub async fn send_response(req: Result<Request, HttpStatusCode>, writer: &mut WriteHalf<'_>) {
    let mut res = Vec::<u8>::new();
    println!("{:?}", req);

    if let Err(status) = req {
        res.extend_from_slice(
            format!("HTTP/1.1 {}\n", status as u32).as_bytes(),
        );
        res.extend_from_slice("Content-Type: text/html\n\n".as_bytes());
        res.extend_from_slice("<h1> An error has occured </h1>".as_bytes());

        writer.write_all(&res).await.unwrap();
        return;
    }

    let req = req.unwrap();

    match fs::File::open(format!("{}{}", DIR, req.path)).await {
        Ok(mut file) => {
            let file_length = file.metadata().await.unwrap().len();
            let mut content = Vec::<u8>::with_capacity(file_length as usize);
            let _ = file.read_to_end(&mut content).await.unwrap();
            let file_type = FileType::from_str(req.path.as_str()).unwrap();

            res.extend_from_slice(format!("HTTP/1.1 {}\n", HttpStatusCode::Ok as u32).as_bytes());
            res.extend_from_slice(
                format!("Content-Type: {}\n", file_type.get_content_type()).as_bytes(),
            );

            if let FileType::Png = file_type {
                res.extend_from_slice(format!("Content-Length: {}\n", file_length).as_bytes());
            }

            res.extend_from_slice("\n".as_bytes());

            res.extend_from_slice(&content);
        }
        Err(_) => {
            res.extend_from_slice(
                format!("HTTP/1.1 {}\n", HttpStatusCode::NotFound as u32).as_bytes(),
            );
            res.extend_from_slice("Content-Type: text/html\n\n".as_bytes());
            res.extend_from_slice("<h1> 404 - Page not found </h1>".as_bytes());
        }
    };

    writer.write_all(&res).await.unwrap();
}
