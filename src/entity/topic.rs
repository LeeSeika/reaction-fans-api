use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    prelude::{async_trait, DateTime},
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, EntityTrait, EnumIter,
    PrimaryKeyTrait, RelationDef, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "topics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: uuid::Uuid,
    pub name: String,
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
