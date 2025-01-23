use crate::errs::http::Error as HttpError;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;

use super::AuthorService;
use crate::entity::author::Column as AuthorColumn;
use crate::entity::author::Entity as AuthorEntity;
use crate::entity::author::Model as AuthorModel;

impl AuthorService {
    pub async fn get_author(&self, aid: Uuid) -> Result<AuthorModel, HttpError> {
        let author = AuthorEntity::find()
            .filter(AuthorColumn::Id.eq(aid))
            .one(self.db.as_ref())
            .await
            .map_err(|e| {
                tklog::error!("Failed to get author, error: ", e);
                HttpError::internal_error(None, None)
            })?
            .ok_or_else(|| HttpError::not_found(None, None))?;
        Ok(author)
    }
}
