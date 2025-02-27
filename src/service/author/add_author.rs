use sea_orm::Set;
use sea_orm::SqlErr;
use sea_orm::{ColumnTrait, EntityTrait};
use uuid::Uuid;

use super::AuthorService;
use crate::api::v1::author::AddAuthorReq;
use crate::constant::Platform;
use crate::entity::author::ActiveModel as AuthorActiveModel;
use crate::entity::author::Column as AuthorColumn;
use crate::entity::author::Entity as AuthorEntity;
use crate::errs::bilibili::Error;
use crate::errs::http::Error as HttpError;
use crate::utils::bilibili::author;
use sea_orm::QueryFilter;

impl AuthorService {
    pub async fn add_author(&self, req: AddAuthorReq) -> Result<(), HttpError> {
        // check if author exists
        let _ = AuthorEntity::find()
            .filter(AuthorColumn::OriginalId.eq(&req.original_id))
            .one(self.db.as_ref())
            .await
            .map_err(|e| {
                tklog::error!("cannot check if author exists, error: ", e);
                HttpError::internal_error(None, None)
            })?
            .ok_or(HttpError::bad_request(None, Some("author already exists")))?;
        // check if author is valid
        let mut name = String::from("");
        match req.platform {
            Platform::Bilibili => {
                let resp = author::get_info(req.original_id.clone())
                    .await
                    .map_err(|e| {
                        tklog::error!("cannot get author info from bilibili, error: ", e);
                        match e {
                            Error::BadRequest(message) => {
                                HttpError::bad_request(None, Some(&message))
                            }
                            Error::NotFound(message) => HttpError::not_found(None, Some(&message)),
                            Error::Unknown(message) => {
                                HttpError::internal_error(None, Some(&message))
                            }
                            _ => HttpError::internal_error(None, None),
                        }
                    })?;
                name = resp.data.name.clone();
            }
            _ => {
                return Err(HttpError::bad_request(None, Some("platform is invalid")));
            }
        }
        // insert author into db
        let author = AuthorActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            original_id: Set(req.original_id),
            platform: Set(req.platform.to_string()),
            space_url: Set(req.space_url),
            ..Default::default()
        };
        let _ = AuthorEntity::insert(author)
            .exec(self.db.as_ref())
            .await
            .map_err(|e| {
                if matches!(e.sql_err(), Some(SqlErr::UniqueConstraintViolation(_))) {
                    HttpError::bad_request(None, Some("author already exists"))
                } else {
                    tklog::error!("cannot insert author into db, error: ", e);
                    HttpError::internal_error(None, None)
                }
            })?;
        Ok(())
    }
}
