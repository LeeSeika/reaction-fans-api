pub struct EmailService {
    pub(super) sender: String,
    pub(super) smtp_username: String,
    pub(super) smtp_pwd: String,
    pub(super) smtp_host: String,
}

pub fn new(
    sender: String,
    smtp_username: String,
    smtp_pwd: String,
    smtp_host: String,
) -> EmailService {
    EmailService {
        sender,
        smtp_username,
        smtp_pwd,
        smtp_host,
    }
}
