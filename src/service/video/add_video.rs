use sea_orm::{ActiveModelTrait, Set, SqlErr, TransactionTrait};
use uuid::Uuid;

use super::VideoService;
use crate::api::v1::video::{AddVideoReq, ResourceType};
use crate::entity::bilibili_meta::ActiveModel as BilibiliMetaActiveModel;
use crate::entity::video::ActiveModel as VideoActiveModel;
use crate::errs::http::Error as HttpError;
use crate::utils::bilibili::{get_iframe_url, get_meta};

impl VideoService {
    pub async fn add_video(&self, req: AddVideoReq) -> Result<(), HttpError> {
        // get bilibili meta from bilibili api
        let meta = get_meta(
            req.resource_id.clone(),
            req.bilibili_resource_type.unwrap_or(ResourceType::Bvid),
        )
        .await
        .map_err(|e| HttpError::bad_request(None, Some(&e.to_string())))?;

        if meta.code != 0 {
            return Err(HttpError::bad_request(None, Some(&meta.message)));
        }

        // new bilibili meta
        let meta_id = Uuid::new_v4();
        let iframe = get_iframe_url(meta.data.aid, meta.data.cid).await;
        let pages = serde_json::to_value(meta.data.pages).map_err(|e| {
            tklog::error!("cannot parse pages, error: ", e);
            HttpError::internal_error(None, Some("cannot parse pages"))
        })?;
        let published_at = chrono::NaiveDateTime::from_timestamp_opt(meta.data.pubdate, 0)
            .unwrap_or_else(|| chrono::NaiveDateTime::from_timestamp(0, 0));
        let bilibili_meta = BilibiliMetaActiveModel {
            id: Set(meta_id),
            bvid: Set(req.resource_id.clone()),
            iframe: Set(iframe),
            videos: Set(meta.data.videos as u64),
            pages: Set(pages),
            pic: Set(meta.data.pic),
            title: Set(meta.data.title),
            pubdate: Set(published_at),
            duration: Set(meta.data.duration as u64),
            view: Set(meta.data.stat.view as u64),
            danmaku: Set(meta.data.stat.danmaku as u64),
            reply: Set(meta.data.stat.reply as u64),
            favorite: Set(meta.data.stat.favorite as u64),
            coin: Set(meta.data.stat.coin as u64),
            share: Set(meta.data.stat.share as u64),
            like: Set(meta.data.stat.like as u64),
            ..Default::default()
        };

        // new video
        let author_id = Uuid::parse_str(&req.author_id)
            .map_err(|e| HttpError::bad_request(None, Some(&e.to_string())))?;
        let topic_id = Uuid::parse_str(&req.topic_id)
            .map_err(|e| HttpError::bad_request(None, Some(&e.to_string())))?;
        let category_id = Uuid::parse_str(&req.category_id)
            .map_err(|e| HttpError::bad_request(None, Some(&e.to_string())))?;

        let video = VideoActiveModel {
            id: Set(Uuid::new_v4()),
            meta_id: Set(meta_id),
            platform: Set(req.platform),
            original_url: Set(Some(req.original_url)),
            author_id: Set(author_id),
            topic_id: Set(topic_id),
            category_id: Set(category_id),
            published_at: Set(published_at),
            ..Default::default()
        };

        // start transaction
        self.db
            .as_ref()
            .transaction::<_, (), sea_orm::DbErr>(|tx| {
                Box::pin(async move {
                    bilibili_meta.insert(tx).await?;
                    video.insert(tx).await?;
                    Ok(())
                })
            })
            .await
            .map_err(|e| match e {
                sea_orm::TransactionError::Connection(db_err) => db_err,
                sea_orm::TransactionError::Transaction(db_err) => db_err,
            })
            .map_err(|e| {
                if let Some(sql_err) = e.sql_err() {
                    if matches!(sql_err, SqlErr::UniqueConstraintViolation(_)) {
                        return HttpError::bad_request(None, Some("video already exists"));
                    }
                }
                tklog::error!("cannot add video, error: ", e);
                HttpError::internal_error(None, None)
            })?;

        Ok(())
    }
}
