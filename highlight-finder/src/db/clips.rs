use mongodb::{
    bson::doc,
    options::{IndexOptions, InsertManyOptions},
    results::InsertManyResult,
};

#[derive(serde::Serialize)]
pub struct Clip {
    pub user_id: String,
    pub vod_id: String,
    pub state: String,
    pub start_time: i64,
    pub end_time: i64,
}

pub async fn save_clips(
    db_client: &mongodb::Database,
    clips: impl IntoIterator<Item = Clip>,
) -> Result<InsertManyResult, mongodb::error::Error> {
    let collection = db_client.collection::<Clip>("clips");

    collection
        .insert_many(clips, InsertManyOptions::builder().ordered(false).build())
        .await
}

pub async fn ensure_clip_unique_index_exists(
    db_client: &mongodb::Database,
) -> Result<mongodb::results::CreateIndexResult, mongodb::error::Error> {
    db_client
        .collection::<Clip>("clips")
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "user_id": 1, "vod_id": 1, "start_time": 1, "end_time": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await
}
