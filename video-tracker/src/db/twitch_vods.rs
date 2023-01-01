use mongodb::{
    bson::doc,
    options::{IndexOptions, InsertManyOptions},
    results::{CreateIndexResult, InsertManyResult},
};

#[derive(serde::Serialize)]
pub struct TwitchVodModel {
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
        .collection::<TwitchVodModel>("twitch_vods")
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
    vods: impl IntoIterator<Item = TwitchVodModel>,
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
