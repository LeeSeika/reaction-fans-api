use actix_web::{web, HttpResponse, Responder};

use crate::AppState;

use super::req_model::AddAuthorReq;

pub async fn add_author(
    state: web::Data<AppState>,
    req: web::Json<AddAuthorReq>,
) -> impl Responder {
    match state.author_service.add_author(req.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}
