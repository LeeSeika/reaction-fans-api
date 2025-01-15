pub mod api;
pub mod conf;
pub mod constant;
pub mod entity;
pub mod errs;
pub mod service;

use std::{ops::DerefMut, sync::Arc};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = conf::Config::from_env().unwrap();

    let server_host = conf.server.host.to_owned();
    let server_port = conf.server.port;

    let conf = Arc::new(conf);

    // init database connection
    let conn = Database::connect(conf.database.url.as_str())
        .await
        .expect("Database connection failed");
    let db = Arc::new(conn);

    // init cache connection
    let client = redis::Client::open(conf.cache.url.as_str()).unwrap();
    // let cache = client
    //     .get_multiplexed_async_connection()
    //     .await
    //     .expect("Cache connection failed");
    let client = Arc::new(client);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                user_service: service::user::new(
                    Arc::clone(&conf),
                    Arc::clone(&db),
                    Arc::clone(&client),
                ),
            }))
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((server_host, server_port))?
    .run()
    .await
}

pub struct AppState {
    pub user_service: service::user::UserService,
}
