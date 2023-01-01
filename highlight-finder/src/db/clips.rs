use mongodb::{options::InsertManyOptions, results::InsertManyResult};

#[derive(serde::Serialize)]
pub struct Clip {
    pub user_id: String,
    pub vod_id: String,
    pub start_time: i64,
    pub end_time: i64,
}

pub async fn save_clips(
    db_client: &mongodb::Database,
    clips: Vec<Clip>,
) -> Result<InsertManyResult, mongodb::error::Error> {
    let collection = db_client.collection::<Clip>("clips");

    collection
        .insert_many(clips, InsertManyOptions::builder().ordered(false).build())
        .await
}
