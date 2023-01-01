use mongodb::bson::{doc, oid::ObjectId};

#[derive(serde::Deserialize, Debug)]
pub struct Clip {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: String,
    pub vod_id: String,
    pub start_time: i64,
    pub end_time: i64,
}

/// Gets a clip that has not been downloaded yet
pub async fn get_pending_clip(
    db_client: &mongodb::Database,
) -> Result<Option<Clip>, mongodb::error::Error> {
    db_client
        .collection::<Clip>("clips")
        .find_one(doc! { "clip_url": { "$exists": false } }, None)
        .await
}
