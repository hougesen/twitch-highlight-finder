use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Database};
use tungstenite::Message;

pub async fn get_db_client() -> Result<Database, mongodb::error::Error> {
    let mongo_uri = dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI");

    let client = Client::with_uri_str(mongo_uri).await?;

    Ok(client.database("highlights"))
}

struct ChannelCollection {
    _id: ObjectId,
    #[allow(dead_code)]
    channel_name: String,
}

pub async fn get_channel_queue(db: &Database) -> Vec<Message> {
    let collection = db.collection::<ChannelCollection>("channels");

    let mut channel_queue = Vec::new();

    if let Ok(channel_queue_bson) = collection.distinct("channel_name", None, None).await {
        if !channel_queue_bson.is_empty() {
            for channel in channel_queue_bson {
                if let Some(channel_name) = channel.as_str() {
                    channel_queue.push(Message::Text(format!(
                        "JOIN #{}",
                        channel_name.to_lowercase().trim()
                    )));
                }
            }
        }
    }

    channel_queue
}
