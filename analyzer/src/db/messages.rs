use mongodb::bson::doc;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TwitchChatMessage {
    pub channel: String,
    pub sender: String,
    pub message: String,
    pub timestamp: mongodb::bson::DateTime,
    pub message_score: f64,
}

pub async fn save_message_batch(
    db_client: &mongodb::Database,
    messages: Vec<TwitchChatMessage>,
) -> Result<mongodb::results::InsertManyResult, mongodb::error::Error> {
    let collection = db_client.collection::<TwitchChatMessage>("twitch_messages");

    collection
        .insert_many(
            messages,
            mongodb::options::InsertManyOptions::builder()
                .ordered(Some(false))
                .build(),
        )
        .await
}

/// Have to make a unique compound index since SQS standard queues can have dupplications
#[allow(unused)]
pub async fn ensure_message_index_exists(
    db_client: &mongodb::Database,
) -> Result<mongodb::results::CreateIndexResult, mongodb::error::Error> {
    let collection = db_client.collection::<TwitchChatMessage>("twitch_messages");

    collection
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "channel": 1, "sender": 1, "message": 1, "timestamp": 1 })
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
