use thiserror::Error;
use reqwest::Error as ReqwestError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(#[from] ReqwestError),
    #[error("Bilibili api error: {0}")]
    BadRequest(String),
    #[error("Bilibili api error: {0}")]
    NotFound(String),
    #[error("Bilibili api error: {0}")]
    Unknown(String),
}

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

pub fn from_code(code: i32, message: String) -> Error {
    match code {
        -400 => Error::BadRequest(message),
        -404 => Error::NotFound(message),
        _ => Error::Unknown(message),
    }
}