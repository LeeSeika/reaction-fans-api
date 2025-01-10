use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}