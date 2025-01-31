
pub async fn get_iframe_url(aid: i64, cid: i64) -> String {
    let url = format!(
        "https://player.bilibili.com/player.html?cid={}&aid={}&page=1&as_wide=1&high_quality=1&danmaku=0",
        cid, aid
    );
    url
}