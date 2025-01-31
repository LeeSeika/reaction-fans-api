use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Cache {
    pub url: String,
}
