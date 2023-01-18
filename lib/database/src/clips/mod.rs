use mongodb::{
    bson::doc,
    options::{IndexOptions, InsertManyOptions},
    results::{CreateIndexResult, InsertManyResult},
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
            mongodb::IndexModel::builder()
                .keys(doc! { "user_id": 1, "vod_id": 1, "start_time": 1, "end_time": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await
}
