use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Mailer {
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_pwd: String,
    pub sender: String,
}
