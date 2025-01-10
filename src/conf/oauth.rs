use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct QQ {
    pub app_id: String,
    pub app_secret: String,
    pub redirect_uri: String,
}