use serde::Deserialize;

#[derive(Deserialize)]
pub struct MatchTopicReq {
    pub(crate) topic: String,
    pub(crate) size: i32,
}

#[derive(Deserialize)]
pub struct AddTopicReq {
    pub(crate) name: String,
}