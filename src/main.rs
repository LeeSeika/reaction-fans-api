pub mod api;
pub mod conf;
pub mod constant;
pub mod entity;
pub mod errs;
pub mod service;
pub mod utils;

use std::sync::Arc;

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

    // init cache client
    let client = redis::Client::open(conf.cache.url.as_str()).expect("Cache client init failed");
    let client = Arc::new(client);
    let topic_service = service::topic::new(Arc::clone(&db)).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                user_service: service::user::new(
                    Arc::clone(&conf),
                    Arc::clone(&db),
                    Arc::clone(&client),
                ),
                video_service: service::video::new(Arc::clone(&db), Arc::clone(&client)),
                author_service: service::author::new(Arc::clone(&db)),
                topic_service: topic_service.clone(),
            }))
            .service(
                web::scope("/api/v1/users")
                    .route("/register", web::post().to(api::v1::user::register))
                    .route(
                        "/verify",
                        web::post().to(api::v1::user::verify_register_code),
                    ),
            )
            .service(
                web::scope("/api/v1/videos")
                    .route("/{id}", web::get().to(api::v1::video::get_video))
                    .route("", web::post().to(api::v1::video::add_video)),
            )
            .service(
                web::scope("/api/v1/authors")
                    .route("", web::post().to(api::v1::author::add_author))
                    .route("/{id}", web::get().to(api::v1::author::get_author)),
            )
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((server_host, server_port))?
    .run()
    .await
}

pub struct AppState {
    pub user_service: service::user::UserService,
    pub video_service: service::video::VideoService,
    pub author_service: service::author::AuthorService,
    pub topic_service: service::topic::TopicService,
}
