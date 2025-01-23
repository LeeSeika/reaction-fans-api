use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::errs::http::Error as HttpError;
use crate::AppState;

pub async fn get_author(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = Uuid::parse_str(&path.into_inner())
        .map_err(|e| HttpError::bad_request(None, Some(&e.to_string())))?;
    match state.author_service.get_author(id).await {
        Ok(author) => Ok(HttpResponse::Ok().json(author)),
        Err(e) => Err(e),
    }
}
