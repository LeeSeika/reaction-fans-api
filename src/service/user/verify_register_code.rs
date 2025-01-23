use super::UserService;
use crate::entity::user;
use crate::entity::user::ActiveModel as UserActiveModel;
use crate::errs::http::Error as HttpError;
use redis::AsyncCommands;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::SqlErr;
use tklog::{error, warn};
use uuid::Uuid;

impl UserService {
    pub async fn verify_register_code(&self, email: String, code: String) -> Result<(), HttpError> {
        let mut cache = self.cache().await.map_err(|e| {
            error!("cannot get redis connection, error: ", e);
            HttpError::internal_error(None, None)
        })?;
        let cache_code = cache
            .get::<String, Option<String>>(email.to_owned())
            .await
            .map_err(|e| {
                error!("cannot get register code from cache, error: ", e);
                HttpError::internal_error(None, None)
            })?;
        if cache_code.is_none() || cache_code.unwrap() != code {
            return Err(HttpError::bad_request(None, Some("code is incorrect")));
        }

        // delete code from cache
        let _ = cache
            .del::<String, ()>(email.to_owned())
            .await
            .map_err(|e| {
                warn!("cannot delete register code from cache, error: ", e);
            });

        let user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(Some(email.to_owned())),
            username: Set(Some("default user".to_owned())),
            ..Default::default()
        };

        // insert user into db
        let insert_result = UserActiveModel::insert(user, self.db.as_ref()).await;

        match insert_result {
            Ok(insert_result) => {
                println!("insert_result: {:?}", insert_result.id);
                Ok(())
            }
            Err(e) => {
                if let Some(sql_err) = e.sql_err() {
                    if matches!(sql_err, SqlErr::UniqueConstraintViolation(_)) {
                        return Err(HttpError::bad_request(None, Some("email already exists")));
                    }
                }
                error!("cannot insert user into db, error: ", e);
                Err(HttpError::internal_error(None, None))
            }
        }
    }
}
