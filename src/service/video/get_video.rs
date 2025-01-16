use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::service::video::svc::VideoService;
use crate::errs::http::Error as HttpError;
use super::VideoEntity;
use super::VideoModel;
use super::VideoColumn;
impl VideoService {
    pub async fn get_video(&self, id: String) -> Result<VideoModel, HttpError> {
        let video = VideoEntity::find()
            .filter(VideoColumn::Id.eq(id.as_str()))
            .one(self.db.as_ref())
            .await
            .map_err(|e| {
                tklog::error!("cannot find video by id, error: ", e);
                HttpError::internal_error(None, None)
            })?
            .ok_or_else(|| HttpError::not_found(None, Some("video not found")))?;

        Ok(video)
    }
}