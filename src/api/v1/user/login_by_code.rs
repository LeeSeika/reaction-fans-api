use super::req_model::LoginByCodeReq;
use actix_web::{web, HttpResponse, Responder};

pub async fn login_by_code(
    state: web::Data<crate::AppState>,
    req_body: web::Json<LoginByCodeReq>,
) -> impl Responder {
    match state
        .user_service
        .login_by_code(req_body.email.clone())
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}
