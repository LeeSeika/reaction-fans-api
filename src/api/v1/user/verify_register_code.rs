use actix_web::{web, HttpResponse, Responder};

use super::req_model::VerifyRegisterCodeReq;

pub async fn verify_register_code(
    state: web::Data<crate::AppState>,
    req_body: web::Json<VerifyRegisterCodeReq>,
) -> impl Responder {
    match state
        .user_service
        .verify_register_code(req_body.email.clone(), req_body.code.clone())
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}
