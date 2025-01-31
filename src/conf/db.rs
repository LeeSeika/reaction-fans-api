use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Database {
    pub url: String,
}