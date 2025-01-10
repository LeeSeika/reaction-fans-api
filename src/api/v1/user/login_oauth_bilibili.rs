use actix_web::{web, HttpResponse, Responder};
use super::req_model::{LoginOauthBilibiliReq};

pub async fn login_oauth_bilibili(
    req_body: web::Json<LoginOauthBilibiliReq>,
) -> impl Responder {
    HttpResponse::Ok()
}