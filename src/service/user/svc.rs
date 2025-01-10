use std::sync::Arc;

use sea_orm::DatabaseConnection;


pub struct UserService {
    db: Arc<DatabaseConnection>
}


