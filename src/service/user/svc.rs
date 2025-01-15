use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::conf;

pub struct UserService {
    pub(super) conf: Arc<conf::Config>,
    pub(super) db: Arc<DatabaseConnection>,
    pub(super) cache: Arc<redis::Client>,
}

pub fn new(
    conf: Arc<conf::Config>,
    db: Arc<DatabaseConnection>,
    cache: Arc<redis::Client>,
) -> UserService {
    UserService { conf, db, cache }
}
