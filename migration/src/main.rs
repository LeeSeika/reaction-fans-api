use migration::Migrator;
use sea_orm::{ConnectOptions, Database};
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    let connect_options = ConnectOptions::new("postgresql://postgres.ynavyropkerchaxytqkc:a626232582@aws-0-ap-southeast-1.pooler.supabase.com:5432/reaction-fans")
        .set_schema_search_path("public") // Override the default schema
        .to_owned();

    let db = Database::connect(connect_options).await.unwrap();

    Migrator::up(&db, None).await.unwrap();
}
