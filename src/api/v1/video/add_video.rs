use actix_web::{web, HttpResponse, Responder};

use crate::api::v1::video::req_model::AddVideoReq;
use crate::AppState;

pub async fn add_video(state: web::Data<AppState>, req: web::Json<AddVideoReq>) -> impl Responder {
    match state.video_service.add_video(req.into_inner()).await {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}
