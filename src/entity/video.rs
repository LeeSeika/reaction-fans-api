use sea_orm::{prelude::{async_trait, DateTime}, ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter, PrimaryKeyTrait, RelationDef, RelationTrait, Set};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use async_trait::async_trait;
use uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "videos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: uuid::Uuid,
    pub name: String,
    pub author_id: i64,
    pub original_url: String,
    pub topic_id: i64,
    pub category_id: i64,
    pub posted_at: DateTime,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        todo!()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, db: &C, insert: bool) -> Result<Self, sea_orm::DbErr>
    where
        C: sea_orm::ConnectionTrait,
    {
        let now = Utc::now().naive_utc();
        let mut clone_self = self.clone();
        if insert {
            clone_self.created_at = Set(now);
        }
        Ok(clone_self)
    }
}