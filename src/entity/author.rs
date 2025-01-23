use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    prelude::{async_trait, DateTime},
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "authors")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: uuid::Uuid,
    pub name: String,
    pub original_id: String,
    pub platform: String,
    pub space_url: Option<String>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::video::Entity")]
    Video,
}

impl Related<super::video::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Video.def()
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
