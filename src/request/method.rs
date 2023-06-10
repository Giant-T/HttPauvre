use std::str::FromStr;

use crate::status::HttpStatusCode;

#[derive(Debug)]
pub enum Method {
    GET,
}

impl FromStr for Method {
    type Err = HttpStatusCode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            _ => Err(HttpStatusCode::NotImplemented),
        }
    }
}

