use std::sync::Arc;

use redis::{aio::MultiplexedConnection, RedisError};
use sea_orm::DatabaseConnection;

use crate::conf;

pub struct UserService {
    pub(super) conf: Arc<conf::Config>,
    pub(super) db: Arc<DatabaseConnection>,
    cache: Arc<redis::Client>,
}

pub fn new(
    conf: Arc<conf::Config>,
    db: Arc<DatabaseConnection>,
    cache: Arc<redis::Client>,
) -> UserService {
    UserService { conf, db, cache }
}

impl UserService {
    pub(super) async fn cache(&self) -> Result<MultiplexedConnection, RedisError> {
        self.cache.as_ref().get_multiplexed_async_connection().await
    }
}
