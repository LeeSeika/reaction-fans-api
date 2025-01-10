use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}