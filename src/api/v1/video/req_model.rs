use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetVideoReq {
    pub(crate) id: String,
}
