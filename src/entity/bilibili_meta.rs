use chrono::Utc;
use sea_orm::{
    prelude::{
        async_trait::{self, async_trait},
        DateTime,
    },
    ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "bilibili_meta")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: uuid::Uuid,
    pub bvid: String,
    pub iframe: String,
    pub videos: u64,
    pub pages: Value,
    pub pic: String,
    pub title: String,
    pub pubdate: DateTime,
    pub duration: u64,
    pub view: u64,
    pub danmaku: u64,
    pub reply: u64,
    pub favorite: u64,
    pub coin: u64,
    pub share: u64,
    pub like: u64,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::video::Entity",
        from = "Column::Id",
        to = "super::video::Column::MetaId"
    )]
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
