use async_trait::async_trait;
use sea_orm::{
    prelude::{async_trait, DateTime},
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait,
};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "videos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: uuid::Uuid,
    pub author_id: uuid::Uuid,
    pub original_url: Option<String>,
    pub topic_id: uuid::Uuid,
    pub category_id: uuid::Uuid,
    pub published_at: DateTime,
    pub platform: String,
    pub meta_id: uuid::Uuid,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::bilibili_meta::Entity")]
    BilibiliMeta,
    #[sea_orm(
        belongs_to = "super::author::Entity",
        from = "Column::AuthorId",
        to = "super::author::Column::Id"
    )]
    Author,
    #[sea_orm(
        belongs_to = "super::topic::Entity",
        from = "Column::TopicId",
        to = "super::topic::Column::Id"
    )]
    Topic,
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
}

impl Related<super::bilibili_meta::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BilibiliMeta.def()
    }
}

impl Related<super::author::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Author.def()
    }
}

impl Related<super::topic::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Topic.def()
    }
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    // async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, sea_orm::DbErr>
    // where
    //     C: sea_orm::ConnectionTrait,
    // {
    //     let now = Utc::now().naive_utc();
    //     let mut clone_self = self.clone();
    //     if insert {
    //         clone_self.created_at = Set(now);
    //     }
    //     Ok(clone_self)
    // }
}
