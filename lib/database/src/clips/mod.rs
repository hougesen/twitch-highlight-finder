use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{IndexOptions, InsertManyOptions},
    results::{CreateIndexResult, InsertManyResult, UpdateResult},
    IndexModel,
};

#[derive(serde::Serialize)]
pub struct PartialClip {
    pub user_id: String,
    pub vod_id: String,
    pub state: String,
    pub start_time: i64,
    pub end_time: i64,
}

pub async fn save_clips(
    db_client: &mongodb::Database,
    clips: impl IntoIterator<Item = PartialClip>,
) -> Result<InsertManyResult, mongodb::error::Error> {
    let collection = db_client.collection::<PartialClip>("clips");

    collection
        .insert_many(clips, InsertManyOptions::builder().ordered(false).build())
        .await
}

pub async fn ensure_clip_unique_index_exists(
    db_client: &mongodb::Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    db_client
        .collection::<PartialClip>("clips")
        .create_index(
            IndexModel::builder()
                .keys(doc! { "user_id": 1, "vod_id": 1, "start_time": 1, "end_time": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await
}

#[derive(serde::Deserialize)]
pub struct Clip {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: String,
    pub vod_id: String,
    #[serde(default = "default_clip_state")]
    pub state: String,
    pub start_time: i64,
    pub end_time: i64,
}

fn default_clip_state() -> String {
    "pending".to_owned()
}

/// Gets a clip that has not been downloaded yet
pub async fn get_pending_clip(
    db_client: &mongodb::Database,
) -> Result<Option<Clip>, mongodb::error::Error> {
    db_client
        .collection::<Clip>("clips")
        .find_one(
            doc! {
                "state": "pending",
                "clip_url": { "$exists": false }
            },
            None,
        )
        .await
}

pub async fn save_video_url(
    db_client: &mongodb::Database,
    id: ObjectId,
    video_url: String,
) -> Result<UpdateResult, mongodb::error::Error> {
    db_client
        .collection::<Clip>("clips")
        .update_one(
            doc! { "_id": id },
            doc! {
                "$set": {
                    "state": "finished",
                    "video_url": video_url
                }
            },
            None,
        )
        .await
}

pub async fn save_clip_state(
    db_client: &mongodb::Database,
    id: ObjectId,
    state: &str,
) -> Result<UpdateResult, mongodb::error::Error> {
    db_client
        .collection::<Clip>("clips")
        .update_one(
            doc! { "_id": id },
            doc! { "$set": { "state": state } },
            None,
        )
        .await
}
