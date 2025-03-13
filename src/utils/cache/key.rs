pub const REGISTER_CODE_PREFIX: &str = "register_code";
pub const LOGIN_CODE_PREFIX: &str = "login_code";

pub fn get_key_with_prefix(prefix: &str, key: &str) -> String {
    format!("{}:{}", prefix, key)
}
