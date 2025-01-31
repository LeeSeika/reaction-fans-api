use std::sync::{PoisonError, RwLockWriteGuard};
use crate::entity::topic::ActiveModel as TopicActiveModel;
use crate::entity::topic::Entity as TopicEntity;
use crate::errs::http::Error as HttpError;
use crate::service::topic::TopicService;
use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, SqlErr};
use uuid::Uuid;

impl TopicService {
    pub async fn add_topic(&self, name: String) -> Result<(), HttpError> {
        let topic = TopicActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name.clone()),
            ..Default::default()
        };

        let insert_result = TopicEntity::insert(topic)
            .exec(self.db.as_ref())
            .await;

        match insert_result {
            Ok(_) => {
                if let Err(_) = self.update_topic_list(name.clone())  {
                    return Err(HttpError::internal_error(None, None));
                }
                Ok(())
            }
            Err(e) => {
                if let Some(sql_err) = e.sql_err() {
                    if matches!(sql_err, SqlErr::UniqueConstraintViolation(_)) {
                        tklog::warn!("topic {} already exists", name);
                        // return HttpError::bad_request(None, Some("topic already exists"));
                        if let Err(_) = self.update_topic_list(name.clone())  {
                            return Err(HttpError::internal_error(None, None));
                        }
                        return Ok(());
                    }
                }
                tklog::error!("cannot add topic, error: ", e);
                Err(HttpError::internal_error(None, None))
            }
        }
    }

    fn update_topic_list(&self, name: String) -> Result<(), PoisonError<RwLockWriteGuard<Vec<String>>>> {
        let mut list = self.topic_list.write().map_err(
            |e| {
                tklog::error!("cannot get RwLock of topic list, error: ", e);
                e
            }
        )?;
        list.push(name);
        Ok(())
    }
}

