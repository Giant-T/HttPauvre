use std::{collections::HashMap, str::FromStr};
use tokio::{fs, io::AsyncReadExt};

use super::file::FileType;
use crate::{request::request::Request, status::HttpStatusCode};

pub const DIR: &str = "www";

pub struct Response {
    pub status: u32,
    headers: HashMap<String, String>,
    pub content: Vec<u8>,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            status: HttpStatusCode::InternalServerError as u32,
            content: Vec::new(),
            headers: HashMap::from([("Server".to_string(), "httpauvre".to_string())]),
        }
    }
}

impl Response {
    ///
    /// Ajoute un header http à la réponse.
    ///
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    ///
    /// Retourne un vecteur correspondant à la réponse en bytes.
    ///
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        result.extend_from_slice(format!("HTTP/1.1 {}\n", self.status).as_bytes());

        for (key, value) in &self.headers {
            result.extend_from_slice(format!("{}: {}\n", key, value).as_bytes());
        }

        result.extend_from_slice("\n".as_bytes());
        result.extend_from_slice(&self.content);

        return result;
    }

    ///
    /// Envoie la réponse au client http.
    ///
    pub async fn from_request(req: Result<Request, HttpStatusCode>) -> Response {
        let mut res = Response::default();

        if let Err(status) = req {
            res.add_header("Content-Type", "text/html");

            if let HttpStatusCode::RequestTimeout = status {
                res.add_header("Connection", "close");
            }

            res.status = status as u32;
            res.content = "<h1> An error has occured </h1>".as_bytes().to_vec();

            return res;
        }

        let req = req.unwrap();

        // TODO: Séparé en plus de fonctions
        match fs::File::open(format!("{}{}", DIR, req.path)).await {
            Ok(mut file) => {
                let file_length = file.metadata().await.unwrap().len();
                let mut content = Vec::<u8>::with_capacity(file_length as usize);

                file.read_to_end(&mut content).await.unwrap();

                let file_type = FileType::from_str(req.path.as_str()).unwrap();
                let file_length = file_length.to_string();

                res.status = HttpStatusCode::Ok as u32;
                res.add_header("Content-Type", file_type.get_content_type());
                res.add_header("Content-Length", file_length.as_str());

                res.content = content;
            }
            Err(_) => {
                res.status = HttpStatusCode::NotFound as u32;
                res.add_header("Content-Type", "text/html");
                res.content = "<h1> 404 - Page not found </h1>".as_bytes().to_vec();
            }
        };

        return res;
    }
}
