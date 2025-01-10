use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "internal error: {}", _0)]
    InternalError(InnerError),

    #[display(fmt = "bad request: {}", _0)]
    BadClientData(InnerError),

    #[display(fmt = "unauthorized: {}", _0)]
    Unauthorized(InnerError),
}

#[derive(Debug, Display, Error)]
#[display(fmt = "(code: {}) {}", code, message)]
pub struct InnerError {
    pub message: String,
    pub code: u16,
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadClientData(_) => StatusCode::BAD_REQUEST,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
}