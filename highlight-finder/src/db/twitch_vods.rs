use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{FindOneOptions, FindOptions},
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TwitchVodModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub vod_id: String,
    pub stream_id: String,
    pub user_id: String,
    pub channel_name: String,
    pub language: String,
    pub title: String,
    pub url: String,
    pub streamed_at: mongodb::bson::DateTime,
    pub ended_at: mongodb::bson::DateTime,
    pub video_duration: u32,
    pub analyzed: bool,
}

pub async fn get_pending_vod(db_client: &mongodb::Database) -> Option<TwitchVodModel> {
    if let Ok(vod) = db_client
        .collection::<TwitchVodModel>("twitch_vods")
        .find_one(
            doc! { "analyzed": false },
            FindOneOptions::builder()
                .sort(doc! { "streamed_at": 1 })
                .build(),
        )
        .await
    {
        return vod;
    }

    None
}

pub async fn get_all_pendings_vods(db_client: &mongodb::Database) -> Vec<TwitchVodModel> {
    if let Ok(cursor) = db_client
        .collection::<TwitchVodModel>("twitch_vods")
        .find(
            doc! { "analyzed": false },
            FindOptions::builder()
                .sort(doc! { "streamed_at": 1 })
                .build(),
        )
        .await
    {
        if let Ok(vods) = cursor.try_collect::<Vec<TwitchVodModel>>().await {
            return vods;
        }
    }

    Vec::new()
}

pub async fn mark_as_analyzed(
    db_client: &mongodb::Database,
    document_id: ObjectId,
) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    db_client
        .collection::<TwitchVodModel>("twitch_vods")
        .update_one(
            doc! { "_id": document_id},
            doc! {
                "$set": {
                    "analyzed": true
                }
            },
            None,
        )
        .await
}
