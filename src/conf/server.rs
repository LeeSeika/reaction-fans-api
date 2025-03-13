use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub secret: String,
    pub salt: String,
    pub issuer: String,
    pub expiration: u64,
}
