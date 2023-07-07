use std::io::ErrorKind;

#[allow(dead_code)]
#[derive(Debug)]
pub enum HttpStatusCode {
    // Successes
    Ok = 200,
    NoContent = 204,

    // Request errors
    BadRequest = 400,
    Forbidden = 403,
    NotFound = 404,
    RequestTimeout = 408,

    // Server errors
    InternalServerError = 500,
    NotImplemented = 501,
    HttpVersionNotSupported = 505,
}

impl From<std::io::Error> for HttpStatusCode {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            ErrorKind::PermissionDenied => HttpStatusCode::Forbidden,
            ErrorKind::NotFound => HttpStatusCode::NotFound,
            _ => HttpStatusCode::InternalServerError,
        }
    }
}
