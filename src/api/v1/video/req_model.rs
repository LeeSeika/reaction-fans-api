use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetVideoReq {
    pub(crate) id: String,
}

#[derive(Deserialize)]
pub enum ResourceType {
    Bvid,
    Aid,
}

#[derive(Deserialize)]
pub struct AddVideoReq {
    pub(crate) resource_id: String,
    pub(crate) platform: String,
    pub(crate) original_url: String,
    pub(crate) author_id: String,
    pub(crate) topic_id: String,
    pub(crate) category_id: String,

    // bilibili
    pub(crate) bilibili_resource_type: Option<ResourceType>,
}
