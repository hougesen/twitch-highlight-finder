use mongodb::{
    bson::doc,
    options::{IndexOptions, InsertManyOptions},
    results::{CreateIndexResult, InsertManyResult},
    {Collection, Database, IndexModel},
};

pub async fn get_db_client() -> Result<Database, mongodb::error::Error> {
    let mongo_uri = dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI");

    let client = mongodb::Client::with_uri_str(mongo_uri).await?;

    Ok(client.database("highlights"))
}

struct Channels {
    #[allow(dead_code)]
    channel_name: String,
    #[allow(dead_code)]
    channel_id: Option<String>,
}

pub async fn fetch_channels() -> Result<Vec<String>, mongodb::error::Error> {
    let db = get_db_client().await?;

    let collection = db.collection::<Channels>("channels");

    let mut channel_id_queue = Vec::new();

    if let Ok(channel_id_queue_bson) = collection.distinct("channel_id", None, None).await {
        if !channel_id_queue_bson.is_empty() {
            for channel in &channel_id_queue_bson {
                if let Some(channel_id) = channel.as_str() {
                    channel_id_queue.push(channel_id.to_string());
                }
            }
        }
    }

    Ok(channel_id_queue)
}
