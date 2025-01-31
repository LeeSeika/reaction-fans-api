use actix_web::{web, HttpResponse, Responder};
use crate::api::v1::topic::req_model::AddTopicReq;
use crate::AppState;

pub async fn add_topic(req: web::Json<AddTopicReq>, state: web::Data<AppState>) -> impl Responder {
    match state.topic_service.add_topic(req.name.clone()).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}
