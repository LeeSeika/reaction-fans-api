use std::sync::Arc;

use crate::entity::user::Entity as UserEntity;
use redis::AsyncCommands;
use sea_orm::{ColumnTrait, EntityOrSelect, EntityTrait, QueryFilter, SelectColumns};
use tklog::{error, warn};

use crate::{
    conf::Config,
    errs::http::Error as HttpError,
    utils::{
        cache::key::{get_key_with_prefix, LOGIN_CODE_PREFIX},
        jwt,
    },
};

use super::UserService;

impl UserService {
    pub async fn verify_login_code(
        &self,
        conf: Arc<Config>,
        email: String,
        code: String,
    ) -> Result<String, HttpError> {
        let mut cache = self.cache().await.map_err(|e| {
            error!("cannot get redis connection, error: ", e);
            HttpError::internal_error(None, None)
        })?;
        let cache_code = cache
            .get::<String, Option<String>>(get_key_with_prefix(LOGIN_CODE_PREFIX, email.as_str()))
            .await
            .map_err(|e| {
                error!("cannot get login code from cache, error: ", e);
                HttpError::internal_error(None, None)
            })?;
        if cache_code.is_none() || cache_code.unwrap() != code {
            return Err(HttpError::bad_request(None, Some("code is incorrect")));
        }

        // delete code from cache
        let _ = cache
            .del::<String, ()>(get_key_with_prefix(LOGIN_CODE_PREFIX, email.as_str()))
            .await
            .map_err(|e| {
                warn!("cannot delete login code from cache, error: ", e);
            });

        // get uid from db
        let uid = UserEntity::find()
            .filter(crate::entity::user::Column::Email.eq(email.as_str()))
            .select_column(crate::entity::user::Column::Id)
            .one(self.db.as_ref())
            .await
            .map_err(|e| {
                error!("cannot get user id from db, error: ", e);
                HttpError::internal_error(None, None)
            })?
            .ok_or_else(|| HttpError::not_found(None, Some("user not found")))?
            .id;

        // gen jwt
        let token = jwt::sign(conf, uid.to_string()).map_err(|e| {
            error!("cannot sign jwt, error: ", e);
            HttpError::internal_error(None, None)
        })?;

        Ok(token)
    }
}
