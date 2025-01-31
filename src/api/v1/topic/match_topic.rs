use actix_web::{web, HttpResponse, Responder};
use crate::api::v1::topic::req_model::MatchTopicReq;

pub async fn match_topic(
    req: web::Query<MatchTopicReq>,
    state: web::Data<crate::AppState>,
) -> impl Responder {
    match state.as_ref().topic_service.match_topic(req.topic.clone(), req.size).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Err(e),
    }
}