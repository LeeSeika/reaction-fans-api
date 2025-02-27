use crate::constant::Platform;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct AddAuthorReq {
    pub original_id: String,
    pub platform: Platform,
    pub space_url: Option<String>,
}
