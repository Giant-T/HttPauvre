use log::{error, info};
use std::{collections::HashMap, str::FromStr, io::Error};
use tokio::{
    fs::{self, File},
    io::AsyncReadExt,
};

use super::file::FileType;
use crate::{request::request::Request, status::HttpStatusCode};

pub const DIR: &str = "www";

pub struct Response {
    pub status: u32,
    headers: HashMap<Box<str>, Box<str>>,
    pub content: Vec<u8>,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            status: HttpStatusCode::InternalServerError as u32,
            content: Vec::new(),
            headers: HashMap::from([(Box::from("Server"), Box::from("httpauvre"))]),
        }
    }
}

impl Response {
    ///
    /// Ajoute un header http à la réponse.
    ///
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(Box::from(key), Box::from(value));
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
    /// Crée la réponse à partir de la requete du client.
    ///
    pub async fn from_request(req: Result<Request, HttpStatusCode>) -> Result<Self, Error> {
        if let Err(status) = req {
            return Ok(Self::generate_error_response(status));
        }

        let req = req.unwrap();

        match fs::File::open(format!("{}{}", DIR, req.path)).await {
            Ok(file) => {
                let res = Self::generate_response_from_file(file, req).await?;
                return Ok(res);
            }
            Err(_) => {
                info!("ressource not found");
                return Ok(Self::generate_error_response(HttpStatusCode::NotFound));
            }
        };
    }

    ///
    /// Genere une reponse a partir d'un fichier
    ///
    async fn generate_response_from_file(mut file: File, req: Request) -> Result<Self, Error> {
        let mut res = Response::default();
        let file_length = file.metadata().await?.len();
        let mut content = Vec::<u8>::with_capacity(file_length as usize);

        file.read_to_end(&mut content).await?;

        let file_type = FileType::from_str(req.path.as_str()).unwrap();
        let file_length = file_length.to_string();

        res.status = HttpStatusCode::Ok as u32;
        res.add_header("Content-Type", file_type.get_content_type());
        res.add_header("Content-Length", file_length.as_str());

        res.content = content;

        return Ok(res);
    }

    ///
    /// Genere une reponse d'erreur
    ///
    pub fn generate_error_response(status: HttpStatusCode) -> Self {
        let mut res = Self::default();
        error!("an error has occured: {:?}", status);

        res.add_header("Content-Type", "text/html");

        res.status = status as u32;
        res.content = "<h1> An error has occured </h1>".as_bytes().to_vec();

        return res;
    }
}
