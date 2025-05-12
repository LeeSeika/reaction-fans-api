#![allow(dead_code)]

use serde::Deserialize;

pub(crate) const REGISTER: &str = "register";
pub(crate) const LOGIN: &str = "login";

pub struct LoginOauthQQReq {
    pub(crate) oauth_code: String,
}

#[derive(Deserialize)]
pub struct LoginByCodeReq {
    pub(crate) email: String,
}

#[derive(Deserialize)]
pub struct VerifyRegisterCodeReq {
    pub(crate) email: String,
    pub(crate) code: String,
}

#[derive(Deserialize)]
pub struct RegisterReq {
    pub(crate) email: String,
}
