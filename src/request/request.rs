use std::{collections::HashMap, str::FromStr};

use tokio::{
    io::{BufReader, AsyncBufReadExt},
    net::tcp::ReadHalf,
};

use crate::status::HttpStatusCode;

use super::method::Method;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
}

impl Request {
    ///
    /// Retourne les informations de la requête http
    ///
    pub async fn from_tcp_reader(
        reader: &mut BufReader<ReadHalf<'_>>,
    ) -> Result<Self, HttpStatusCode> {
        let mut args = String::new();

        reader.read_line(&mut args).await.unwrap();

        let mut args = args.split(" ");

        let method = args.next();
        let path = args.next();
        let protocol_version = args.next();

        let (Some(_), Some(_), Some(_)) = (method, path, protocol_version) else {
            return Err(HttpStatusCode::BadRequest);
        };

        let method = Method::from_str(method.unwrap());
        let mut path = path.unwrap().to_string();
        let protocol_version = protocol_version.unwrap().trim();

        if protocol_version != "HTTP/1.1" {
            return Err(HttpStatusCode::HttpVersionNotSupported);
        }

        let mut buf = String::new();
        
        // src : https://stackoverflow.com/questions/54094037/how-can-a-web-server-know-when-an-http-request-is-fully-received
        while !buf.ends_with("\r\n\r\n") {
            reader.read_line(&mut buf).await.unwrap();
        }

        if path.ends_with("/") {
            path += "index.html";
        }
        println!("{}", buf);

        let mut headers: HashMap<String, String> = HashMap::new();

        if buf.lines().count() > 2 {
            headers = Self::parse_headers(buf);
        }

        if let Ok(method) = method {
            return Ok(Request {
                path,
                method,
                headers,
            });
        }

        return Err(method.unwrap_err());
    }

    ///
    /// Parse les headers et construit une hashmap de clef et de valeurs
    /// qui represente les headers de la requete.
    ///
    fn parse_headers(buf: String) -> HashMap<String, String> {
        return buf
            .lines()
            .collect::<Vec<_>>()
            .split_last()
            .unwrap()
            .1
            .iter()
            .map(|s| {
                let h = s.split_once(": ").unwrap();
                return (h.0.to_string(), h.1.to_string());
            })
            .collect();
    }
}
