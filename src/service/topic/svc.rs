use crate::entity::topic::Entity as TopicEntity;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct TopicService {
    pub(super) db: Arc<DatabaseConnection>,
    pub(super) topic_list: Arc<RwLock<Vec<String>>>,
}

pub async fn new(db: Arc<DatabaseConnection>) -> TopicService {
    let list = TopicEntity::find()
        .all(db.as_ref())
        .await
        .unwrap()
        .into_iter();

    let mut topic_list = vec![];
    for topic in list {
        topic_list.push(topic.name);
    }
    let topic_list = Arc::new(RwLock::new(topic_list));

    TopicService { db, topic_list }
}
