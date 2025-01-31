use std::sync::Arc;

use sea_orm::DatabaseConnection;

pub struct AuthorService {
    pub(super) db: Arc<DatabaseConnection>,
}

pub fn new(db: Arc<DatabaseConnection>) -> AuthorService {
    AuthorService { db }
}
