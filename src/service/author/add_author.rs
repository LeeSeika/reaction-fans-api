use sea_orm::EntityTrait;
use sea_orm::Set;
use sea_orm::SqlErr;
use uuid::Uuid;

use super::AuthorService;
use crate::api::v1::author::AddAuthorReq;
use crate::entity::author::ActiveModel as AuthorActiveModel;
use crate::entity::author::Entity as AuthorEntity;
use crate::errs::http::Error as HttpError;

impl AuthorService {
    pub async fn add_author(&self, req: AddAuthorReq) -> Result<(), HttpError> {
        let author = AuthorActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(req.name),
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
