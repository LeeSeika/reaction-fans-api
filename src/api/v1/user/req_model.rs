use serde::Deserialize;

pub struct LoginOauthQQReq {
    pub(crate) oauth_code: String,
}

#[derive(Deserialize)]
pub struct RegisterReq {
    pub(crate) email: String,
}
