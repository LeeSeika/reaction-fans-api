use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};

use super::req_model::{VerifyRegisterCodeReq, LOGIN, REGISTER};

pub async fn verify_code(
    state: web::Data<crate::AppState>,
    req_body: web::Json<VerifyRegisterCodeReq>,
    biz: web::Path<String>,
) -> impl Responder {
    match biz.into_inner().as_str() {
        REGISTER => {
            let result = state
                .user_service
                .verify_register_code(req_body.email.clone(), req_body.code.clone())
                .await;
            match result {
                Ok(_) => Ok(HttpResponse::Ok().finish()),
                Err(e) => Err(e),
            }
        }
        LOGIN => {
            let result = state
                .user_service
                .verify_login_code(
                    state.conf.clone(),
                    req_body.email.clone(),
                    req_body.code.clone(),
                )
                .await;
            match result {
                Ok(token) => Ok(HttpResponse::Ok().json(HashMap::from([("token", token)]))),
                Err(e) => Err(e),
            }
        }
        _ => Err(crate::errs::http::Error::bad_request(
            None,
            Some("invalid biz type"),
        )),
    }
}
