use redis::AsyncCommands;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};

use super::svc::UserService;
use super::UserColumn;
use super::UserEntity;
use crate::constant::REGISTER_CODE_EXPIRE_TIME;
use crate::errs::http::Error as HttpError;
use crate::service::email;
use tklog::error;

impl UserService {
    pub async fn register(&self, email: String) -> Result<(), HttpError> {
        match UserEntity::find()
            .filter(UserColumn::Email.eq(email.as_str()))
            .one(self.db.as_ref())
            .await
            .map_err(|e| {
                error!("cannot find user by email, error: ", e);
                HttpError::internal_error(None, None)
            })? {
            Some(_) => return Err(HttpError::bad_request(None, Some("email already exists"))),
            None => {}
        };

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
                email.to_owned(),
                code.to_owned(),
                REGISTER_CODE_EXPIRE_TIME,
            )
            .await
            .map_err(|e| {
                error!("cannot set code to cache, error: ", e);
                HttpError::internal_error(None, None)
            })?;

        // TODO make it async
        // send email asynchronously
        self.send_register_code(email, code).await;

        Ok(())
    }

    async fn send_register_code(&self, email: String, code: String) {
        let email_service = email::new(
            self.conf.mailer.sender.to_owned(),
            self.conf.mailer.smtp_username.to_owned(),
            self.conf.mailer.smtp_pwd.to_owned(),
            self.conf.mailer.smtp_host.to_owned(),
        );
        email_service
            .send_mail(email, "Register Code".to_owned(), code)
            .await;
    }
}
