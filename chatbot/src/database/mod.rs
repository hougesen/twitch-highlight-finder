use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Database};

pub async fn get_db_client() -> Result<Database, mongodb::error::Error> {
    let mongo_uri = dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI");

    let client = Client::with_uri_str(mongo_uri).await?;

    Ok(client.database("highlights"))
}

struct ChannelCollection {
    _id: ObjectId,
    #[allow(dead_code)]
    channel_id: String,
}

pub async fn get_db_channels() -> Vec<String> {
    let mut channel_queue = Vec::new();

    if let Ok(db) = get_db_client().await {
        let collection = db.collection::<ChannelCollection>("channels");

        if let Ok(channel_queue_bson) = collection.distinct("channel_id", None, None).await {
            if !channel_queue_bson.is_empty() {
                for channel in channel_queue_bson {
                    if let Some(channel_id) = channel.as_str() {
                        channel_queue.push(channel_id.to_string());
                    }
                }
            }
        }
    }

    channel_queue
}
