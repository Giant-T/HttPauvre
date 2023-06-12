#[allow(dead_code)]
#[derive(Debug)]
pub enum HttpStatusCode {
    // Successes
    Ok = 200,
    NoContent = 204,
    
    // Request errors
    BadRequest = 400,
    NotFound = 404,

    // Server errors
    InternalServerError = 500,
    NotImplemented = 501,
    HttpVersionNotSupported = 505,
}
