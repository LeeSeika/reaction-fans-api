use crate::api::v1::video::ResourceType;
use serde::{Deserialize, Serialize};

pub async fn get_meta(
    resource_id: String,
    resource_type: ResourceType,
) -> Result<BilibiliResponse, Box<dyn std::error::Error>> {
    match resource_type {
        ResourceType::Bvid => {
            let bvid = resource_id.clone();
            let url = format!(
                "https://api.bilibili.com/x/web-interface/view?bvid={}",
                bvid
            );
            let resp = reqwest::get(url).await?.json::<BilibiliResponse>().await?;
            Ok(resp)
        }
        ResourceType::Aid => {
            let aid = resource_id.parse::<i64>()?;
            let url = format!("https://api.bilibili.com/x/web-interface/view?aid={}", aid);
            let resp = reqwest::get(url).await?.json::<BilibiliResponse>().await?;
            Ok(resp)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BilibiliResponse {
    pub code: i32,
    pub message: String,
    pub ttl: i32,
    pub data: BilibiliMetaData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BilibiliMetaData {
    pub bvid: String,
    pub aid: i64,
    pub videos: i32,
    pub tid: i32,
    pub tid_v2: i32,
    pub tname: String,
    pub tname_v2: String,
    pub copyright: i32,
    pub pic: String,
    pub title: String,
    pub pubdate: i64,
    pub ctime: i64,
    pub desc: String,
    pub desc_v2: Option<String>,
    pub state: i32,
    pub duration: i32,
    pub rights: Rights,
    pub owner: Owner,
    pub stat: Stat,
    pub argue_info: ArgueInfo,
    pub dynamic: String,
    pub cid: i64,
    pub dimension: Dimension,
    pub premiere: Option<String>,
    pub teenage_mode: i32,
    pub is_chargeable_season: bool,
    pub is_story: bool,
    pub is_upower_exclusive: bool,
    pub is_upower_play: bool,
    pub is_upower_preview: bool,
    pub enable_vt: i32,
    pub vt_display: String,
    pub no_cache: bool,
    pub pages: Vec<Page>,
    pub subtitle: Subtitle,
    pub is_season_display: bool,
    pub user_garb: UserGarb,
    pub honor_reply: serde_json::Value, // 使用 Value 因为是空对象
    pub like_icon: String,
    pub need_jump_bv: bool,
    pub disable_show_up_info: bool,
    pub is_story_play: i32,
    pub is_view_self: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rights {
    pub bp: i32,
    pub elec: i32,
    pub download: i32,
    pub movie: i32,
    pub pay: i32,
    pub hd5: i32,
    pub no_reprint: i32,
    pub autoplay: i32,
    pub ugc_pay: i32,
    pub is_cooperation: i32,
    pub ugc_pay_preview: i32,
    pub no_background: i32,
    pub clean_mode: i32,
    pub is_stein_gate: i32,
    pub is_360: i32,
    pub no_share: i32,
    pub arc_pay: i32,
    pub free_watch: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub mid: i64,
    pub name: String,
    pub face: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    pub aid: i64,
    pub view: i32,
    pub danmaku: i32,
    pub reply: i32,
    pub favorite: i32,
    pub coin: i32,
    pub share: i32,
    pub now_rank: i32,
    pub his_rank: i32,
    pub like: i32,
    pub dislike: i32,
    pub evaluation: String,
    pub vt: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArgueInfo {
    pub argue_msg: String,
    pub argue_type: i32,
    pub argue_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dimension {
    pub width: i32,
    pub height: i32,
    pub rotate: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub cid: i64,
    pub page: i32,
    pub from: String,
    pub part: String,
    pub duration: i32,
    pub vid: String,
    pub weblink: String,
    pub dimension: Dimension,
    pub first_frame: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subtitle {
    pub allow_submit: bool,
    pub list: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGarb {
    pub url_image_ani_cut: String,
}