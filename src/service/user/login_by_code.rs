use redis::AsyncCommands;
use tklog::error;

use super::UserService;
use crate::{
    constant::LOGIN_CODE_EXPIRE_TIME,
    errs::http::Error as HttpError,
    service::email,
    utils::cache::key::{get_key_with_prefix, LOGIN_CODE_PREFIX},
};

impl UserService {
    pub async fn login_by_code(&self, email: String) -> Result<(), HttpError> {
        // gen a random 4-digit number
        let code = format!("{:04}", rand::random::<u16>() % 10000);

        // set code to cache
        self.cache()
            .await
            .map_err(|e| {
                error!("cannot get redis connection, error: ", e);
                HttpError::internal_error(None, None)
            })?
            .set_ex::<String, String, ()>(
                get_key_with_prefix(LOGIN_CODE_PREFIX, email.as_str()),
                code.to_owned(),
                LOGIN_CODE_EXPIRE_TIME,
            )
            .await
            .map_err(|e| {
                error!("cannot set code to cache, error: ", e);
                HttpError::internal_error(None, None)
            })?;

        // TODO make it async
        // send email asynchronously
        self.send_login_code(email, code).await;

        Ok(())
    }

    async fn send_login_code(&self, email: String, code: String) {
        let email_service = email::new(
            self.conf.mailer.sender.to_owned(),
            self.conf.mailer.smtp_username.to_owned(),
            self.conf.mailer.smtp_pwd.to_owned(),
            self.conf.mailer.smtp_host.to_owned(),
        );
        email_service
            .send_mail(email, "Login Code".to_owned(), code)
            .await;
    }
}
