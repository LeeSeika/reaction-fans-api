use std::sync::Arc;
use redis::aio::MultiplexedConnection;
use redis::RedisError;
use sea_orm::DatabaseConnection;

pub struct VideoService {
    pub(super) db: Arc<DatabaseConnection>,
    cache: Arc<redis::Client>,
}

pub fn new(
    db: Arc<DatabaseConnection>,
    cache: Arc<redis::Client>,
) -> VideoService {
    VideoService { db, cache }
}

impl VideoService {
    pub(super) async fn cache(&self) -> Result<MultiplexedConnection, RedisError> {
        self.cache.as_ref().get_multiplexed_async_connection().await
    }
}