use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use super::req_model::LoginOauthQQReq;

pub async fn login_oauth_qq(
    req_body: web::Json<LoginOauthQQReq>,
) -> impl Responder {
    HttpResponse::Ok()
}