use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    options::{IndexOptions, InsertManyOptions},
    results::{CreateIndexResult, InsertManyResult},
};

#[derive(serde::Serialize)]
pub struct PartialTwitchVodModel {
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

async fn ensure_video_index_exists(
    db_client: &mongodb::Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    db_client
        .collection::<PartialTwitchVodModel>("twitch_vods")
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "vod_id": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await
}

pub async fn save_vods(
    db_client: &mongodb::Database,
    vods: impl IntoIterator<Item = PartialTwitchVodModel>,
) -> Result<InsertManyResult, mongodb::error::Error> {
    ensure_video_index_exists(db_client).await?;

    db_client
        .collection("twitch_vods")
        .insert_many(
            vods,
            InsertManyOptions::builder().ordered(Some(false)).build(),
        )
        .await
}

#[derive(serde::Serialize, serde::Deserialize)]
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

pub async fn get_pending_vod(
    db_client: &mongodb::Database,
) -> Result<Option<TwitchVodModel>, mongodb::error::Error> {
    db_client
        .collection::<TwitchVodModel>("twitch_vods")
        .find_one(doc! { "analyzed": false }, None)
        .await
}

pub async fn get_all_pendings_vods(db_client: &mongodb::Database) -> Vec<TwitchVodModel> {
    if let Ok(cursor) = db_client
        .collection::<TwitchVodModel>("twitch_vods")
        .find(doc! { "analyzed": false }, None)
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
    document_id: &ObjectId,
) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    db_client
        .collection::<TwitchVodModel>("twitch_vods")
        .update_one(
            doc! { "_id": document_id },
            doc! {
                "$set": {
                    "analyzed": true,
                    "last_updated": DateTime::now()
                }
            },
            None,
        )
        .await
}
