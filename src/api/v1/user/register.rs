use super::req_model::RegisterReq;
use actix_web::{web, HttpResponse, Responder};

pub async fn register(
    state: web::Data<crate::AppState>,
    req_body: web::Json<RegisterReq>,
) -> impl Responder {
    state.user_service.register(req_body.email.clone()).await;
    HttpResponse::Ok().finish()
}
