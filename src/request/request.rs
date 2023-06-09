use std::{collections::HashMap, str::FromStr};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::tcp::ReadHalf,
};

use crate::status::HttpStatusCode;

use super::method::Method;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<Box<str>, Box<str>>,
}

impl Request {
    ///
    /// Retourne les informations de la requête http
    ///
    pub async fn parse_request(
        mut reader: BufReader<ReadHalf<'_>>,
    ) -> Result<Self, HttpStatusCode> {
        let mut args = String::new();

        reader.read_line(&mut args).await?;

        let args = Self::parse_method_path_protocol(args)?;

        let (method, mut path, protocol_version) = args;
        let method = method?;

        if protocol_version != "HTTP/1.1" {
            return Err(HttpStatusCode::HttpVersionNotSupported);
        }

        let mut buf = String::new();

        // src : https://stackoverflow.com/questions/54094037/how-can-a-web-server-know-when-an-http-request-is-fully-received
        while !buf.ends_with("\r\n\r\n") {
            reader.read_line(&mut buf).await?;
        }

        if path.ends_with("/") {
            path += "index.html";
        }

        let headers = Self::parse_headers(buf).unwrap_or_default();

        return Ok(Request {
            path,
            method,
            headers,
        });
    }

    ///
    /// Parse la methode, le chemin ainsi que le protocole utilisé lors de la
    /// requête envoyé par le client.
    ///
    fn parse_method_path_protocol(
        buf: String,
    ) -> Result<(Result<Method, HttpStatusCode>, String, String), HttpStatusCode> {
        let mut args = buf.split(" ");

        let method = args.next();
        let path = args.next();
        let protocol_version = args.next();

        if let (Some(method), Some(path), Some(protocol_version)) = (method, path, protocol_version)
        {
            return Ok((
                Method::from_str(method),
                path.to_string(),
                protocol_version.trim().to_string(),
            ));
        }

        return Err(HttpStatusCode::BadRequest);
    }

    ///
    /// Parse les headers et construit une hashmap de clef et de valeurs
    /// qui represente les headers de la requete.
    ///
    fn parse_headers(buf: String) -> Option<HashMap<Box<str>, Box<str>>> {
        return Some(
            buf.lines()
                .collect::<Vec<_>>()
                .split_last()?
                .1
                .iter()
                .filter_map(|s| {
                    let h = s.split_once(": ")?;
                    return Some((Box::from(h.0), Box::from(h.1)));
                })
                .collect(),
        );
    }
}
