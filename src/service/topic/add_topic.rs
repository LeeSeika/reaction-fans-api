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
            name: Set(name),
            ..Default::default()
        };

        TopicEntity::insert(topic)
            .exec(self.db.as_ref())
            .await
            .map_err(|e| {
                if let Some(sql_err) = e.sql_err() {
                    if matches!(sql_err, SqlErr::UniqueConstraintViolation(_)) {
                        return HttpError::bad_request(None, Some("topic already exists"));
                    }
                }
                tklog::error!("cannot add topic, error: ", e);
                HttpError::internal_error(None, None)
            })?;

        Ok(())
    }
}
