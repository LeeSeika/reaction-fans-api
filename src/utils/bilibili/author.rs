use crate::utils::bilibili::sign;
use serde::Deserialize;
use crate::errs;

pub async fn get_info(mid: String) -> Result<BilibiliResponse, errs::bilibili::Error> {
    let url_encoded_params = sign::wbi(vec![("mid", mid.clone())]).await;
    let req_url = format!(
        "{}?{}",
        "https://api.bilibili.com/x/space/wbi/acc/info", url_encoded_params
    );

    let client = reqwest::Client::new();
    let resp = client.get(req_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/")
        .header("Referer", "https://www.bilibili.com/")
        .header("Cookie", "buvid4=A5969146-EA42-4811-BA1F-9AE00F651C4426444-022100915-0mOPZbinCsMIHWdkhrNF6Q%3D%3D; buvid_fp_plain=undefined; share_source_origin=WEIXIN; theme_style=light; FEED_LIVE_VERSION=V8; header_theme_version=CLOSE; rpdid=|(u~km)k)m~m0J'u~|RRJY|YY; enable_web_push=DISABLE; home_feed_column=5; bsource=search_google; LIVE_BUVID=AUTO5017077511515282; CURRENT_QUALITY=120; _ga=GA1.1.659939330.1668498701; _ga_34B604LFFQ=GS1.1.1714999668.50.1.1715000067.60.0.0; buvid3=2572106E-7763-EC81-0E65-ECFD3BE6411265437infoc; b_nut=1737211865; _uuid=B65293FA-910EC-411D-5995-A95D7F757107E65634infoc; enable_feed_channel=DISABLE; browser_resolution=1470-798; SESSDATA=3e43a6bb%2C1753623741%2C2c08e%2A12CjC7W3bZ3VMrMQw4P-tO65TqTHS4gK052YKo5cuOQs6gRTs1DibjIkXu-BCwJBDJZJcSVklNOUJtMHVBOFRGQmRJSVZsSGZTalhmZExsa3c4SksxUERPOERnUDE2dktYWTM4ZXdiWGZPcFlndWVWY0dqTlMxV1NLVmc5a1BXNGV3cXJyT0VlekNRIIEC; bili_jct=43b013f51f5f6812d9db78af6ea74f90; DedeUserID=96143142; DedeUserID__ckMd5=ffaae1a2b78d4e99; CURRENT_FNVAL=2000; bili_ticket=eyJhbGciOiJIUzI1NiIsImtpZCI6InMwMyIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MzgzMzA5NzMsImlhdCI6MTczODA3MTcxMywicGx0IjotMX0.A3XViSNTwSPvD-ObIGkGj2fzt-TJOVVpkSMSdA6CyAw; bili_ticket_expires=1738330913; sid=puz3wgo5; bp_t_offset_96143142=1027524677360156672; fingerprint=afcf9e8c3e43f637f15629b83ff8d995; PVID=3; buvid_fp=2572106E-7763-EC81-0E65-ECFD3BE6411265437infoc; b_lsid=8DA110D49_194B0894640")
        .send()
        .await?
        .json::<BilibiliResponse>()
        .await?;

    if resp.code != 0 {
        return Err(errs::bilibili::from_code(resp.code, resp.message));
    }

    Ok(resp)
}

#[derive(Deserialize, Debug)]
pub struct BilibiliResponse {
    pub code: i32,
    pub message: String,
    pub ttl: i32,
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub mid: i64,
    pub name: String,
    pub sex: String,
    pub face: String,
    pub face_nft: i32,
    pub face_nft_type: i32,
    pub sign: String,
    pub rank: i32,
    pub level: i32,
    pub jointime: i32,
    pub moral: i32,
    pub silence: i32,
    pub coins: f64,
    pub fans_badge: bool,
    pub fans_medal: Option<FansMedal>,
    pub official: Official,
    pub vip: Vip,
    pub pendant: Pendant,
    pub nameplate: Nameplate,
    pub user_honour_info: UserHonourInfo,
    pub is_followed: bool,
    pub top_photo: String,
    pub sys_notice: SysNotice,
    pub live_room: LiveRoom,
    pub birthday: String,
    pub school: School,
    pub profession: Profession,
    pub tags: Option<Vec<String>>,
    pub series: Series,
    pub is_senior_member: i32,
    pub mcn_info: Option<String>,
    pub gaia_res_type: i32,
    pub gaia_data: Option<String>,
    pub is_risk: bool,
    pub elec: Elec,
    pub contract: Contract,
    pub certificate_show: bool,
    pub name_render: Option<String>,
    pub top_photo_v2: TopPhotoV2,
    pub theme: Option<String>,
    pub attestation: Attestation,
}

#[derive(Deserialize, Debug)]
pub struct FansMedal {
    pub show: bool,
    pub wear: bool,
    pub medal: Medal,
}

#[derive(Deserialize, Debug)]
pub struct Medal {
    pub uid: i64,
    pub target_id: i64,
    pub medal_id: i64,
    pub level: i32,
    pub medal_name: String,
    pub medal_color: i32,
    pub intimacy: i32,
    pub next_intimacy: i32,
    pub day_limit: i32,
    pub medal_color_start: i32,
    pub medal_color_end: i32,
    pub medal_color_border: i32,
    pub is_lighted: i32,
    pub light_status: i32,
    pub wearing_status: i32,
    pub score: i32,
}

#[derive(Deserialize, Debug)]
pub struct Official {
    pub role: i32,
    pub title: String,
    pub desc: String,
    pub r#type: i32,
}

#[derive(Deserialize, Debug)]
pub struct Vip {
    pub r#type: i32,
    pub status: i32,
    pub due_date: i64,
    pub vip_pay_type: i32,
    pub theme_type: i32,
    pub label: Label,
    pub avatar_subscript: i32,
    pub nickname_color: String,
    pub role: i32,
    pub avatar_subscript_url: String,
    pub tv_vip_status: i32,
    pub tv_vip_pay_type: i32,
    pub tv_due_date: i64,
    pub avatar_icon: AvatarIcon,
}

#[derive(Deserialize, Debug)]
pub struct Label {
    pub path: String,
    pub text: String,
    pub label_theme: String,
    pub text_color: String,
    pub bg_style: i32,
    pub bg_color: String,
    pub border_color: String,
    pub use_img_label: bool,
    pub img_label_uri_hans: String,
    pub img_label_uri_hant: String,
    pub img_label_uri_hans_static: String,
    pub img_label_uri_hant_static: String,
}

#[derive(Deserialize, Debug)]
pub struct AvatarIcon {
    pub icon_type: i32,
    pub icon_resource: IconResource,
}

#[derive(Deserialize, Debug)]
pub struct IconResource {
    pub r#type: i32,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Pendant {
    pub pid: i32,
    pub name: String,
    pub image: String,
    pub expire: i64,
    pub image_enhance: String,
    pub image_enhance_frame: String,
    pub n_pid: i32,
}

#[derive(Deserialize, Debug)]
pub struct Nameplate {
    pub nid: i32,
    pub name: String,
    pub image: String,
    pub image_small: String,
    pub level: String,
    pub condition: String,
}

#[derive(Deserialize, Debug)]
pub struct UserHonourInfo {
    pub mid: i64,
    pub colour: Option<String>,
    pub tags: Vec<String>,
    pub is_latest_100honour: i32,
}

#[derive(Deserialize, Debug)]
pub struct SysNotice {}

#[derive(Deserialize, Debug)]
pub struct LiveRoom {
    pub roomStatus: i32,
    pub liveStatus: i32,
    pub url: String,
    pub title: String,
    pub cover: String,
    pub roomid: i64,
    pub roundStatus: i32,
    pub broadcast_type: i32,
    pub watched_show: WatchedShow,
}

#[derive(Deserialize, Debug)]
pub struct WatchedShow {
    pub switch: bool,
    pub num: i32,
    pub text_small: String,
    pub text_large: String,
    pub icon: String,
    pub icon_location: String,
    pub icon_web: String,
}

#[derive(Deserialize, Debug)]
pub struct School {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Profession {
    pub name: String,
    pub department: String,
    pub title: String,
    pub is_show: i32,
}

#[derive(Deserialize, Debug)]
pub struct Series {
    pub user_upgrade_status: i32,
    pub show_upgrade_window: bool,
}

#[derive(Deserialize, Debug)]
pub struct Elec {
    pub show_info: ShowInfo,
}

#[derive(Deserialize, Debug)]
pub struct ShowInfo {
    pub show: bool,
    pub state: i32,
    pub title: String,
    pub icon: String,
    pub jump_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Contract {
    pub is_display: bool,
    pub is_follow_display: bool,
}

#[derive(Deserialize, Debug)]
pub struct TopPhotoV2 {
    pub sid: i64,
    pub l_img: String,
    pub l_200h_img: String,
}

#[derive(Deserialize, Debug)]
pub struct Attestation {
    pub r#type: i32,
    pub common_info: CommonInfo,
    pub splice_info: SpliceInfo,
    pub icon: String,
    pub desc: String,
}

#[derive(Deserialize, Debug)]
pub struct CommonInfo {
    pub title: String,
    pub prefix: String,
    pub prefix_title: String,
}

#[derive(Deserialize, Debug)]
pub struct SpliceInfo {
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_info() {
        let mid = "96143142".to_string();
        let _ = get_info(mid).await.unwrap();
    }
}
