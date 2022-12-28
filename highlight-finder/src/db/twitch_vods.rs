use mongodb::{bson::doc, options::FindOneOptions, results::CreateIndexResult};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

pub async fn ensure_video_index_exists(
    db_client: &mongodb::Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    db_client
        .collection::<TwitchVodModel>("twitch_vods")
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "vod_id": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build(),
                )
                .build(),
            None,
        )
        .await
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
