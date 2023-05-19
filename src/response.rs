use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::WriteHalf,
};

use crate::request::{FileType, Request};

pub const DIR: &str = "www";

pub enum HttpCode {
    HttpOk = 200,
    HttpNotFound = 404,
}

///
/// Envoie la réponse au client http.
///
pub async fn send_response(req: Request, writer: &mut WriteHalf<'_>) {
    let mut res = Vec::<u8>::new();

    match fs::File::open(format!("{}/{}", DIR, req.file_name)).await {
        Ok(mut file) => {
            let file_length = file.metadata().await.unwrap().len();
            let mut content = Vec::<u8>::with_capacity(file_length as usize);
            let _ = file.read_to_end(&mut content).await.unwrap();

            res.extend_from_slice(format!("HTTP/1.1 {}\n", HttpCode::HttpOk as u32).as_bytes());
            res.extend_from_slice(
                format!("Content-Type: {}\n", req.file_type.get_content_type()).as_bytes(),
            );
            if let FileType::Png = req.file_type {
                res.extend_from_slice(format!("Content-Length: {}\n", file_length).as_bytes());
            }

            res.extend_from_slice("\n".as_bytes());

            res.extend_from_slice(&content);
        }
        Err(_) => {
            res.extend_from_slice(
                format!("HTTP/1.1 {}\n", HttpCode::HttpNotFound as u32).as_bytes(),
            );
            res.extend_from_slice("Content-Type: text/html\n\n".as_bytes());
            res.extend_from_slice("<h1> 404 - Page not found </h1>".as_bytes());
        }
    };

    writer.write_all(&res).await.unwrap();
}
