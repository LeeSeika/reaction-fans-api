use actix_web::{web, HttpResponse, Responder};

use super::req_model::GetVideoReq;

pub async fn get_video(
    state: web::Data<crate::AppState>,
    path: web::Path<GetVideoReq>,
) -> impl Responder {
    match state.video_service.get_video(path.id.clone()).await {
        Ok(video) => Ok(HttpResponse::Ok().json(video)),
        Err(e) => Err(e),
    }
}
