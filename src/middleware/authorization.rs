use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web, Error,
};

use crate::{
    errs::{self, http::Error as HttpError},
    utils, AppState,
};

pub async fn authorization(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let token = req.headers().get("Authorization");
    if token.is_none() {
        return Err(Error::from(HttpError::unauthorized(
            None,
            Some("invalid token"),
        )));
    }

    let token = token
        .unwrap()
        .to_str()
        .map_err(|_| Error::from(HttpError::unauthorized(None, Some("invalid token"))))?;

    let token = token
        .split("Bearer ")
        .last()
        .ok_or_else(|| Error::from(HttpError::unauthorized(None, Some("invalid token"))))?;

    let conf = req.app_data::<web::Data<AppState>>().unwrap().conf.clone();

    let _ = utils::jwt::verify(conf, token.to_string()).map_err(|e| match e {
        errs::jwt::Error::Invalid => {
            Error::from(HttpError::unauthorized(None, Some("invalid token")))
        }
        errs::jwt::Error::Unexpected(cause) => {
            tklog::error!("cannot create hmac key, error: ", cause);
            Error::from(HttpError::internal_error(None, None))
        }
    });

    next.call(req).await
}
