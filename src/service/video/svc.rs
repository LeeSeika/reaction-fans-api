use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct VideoService {
    pub(super) db: Arc<DatabaseConnection>,
}

pub fn new(db: Arc<DatabaseConnection>) -> VideoService {
    VideoService { db }
}
