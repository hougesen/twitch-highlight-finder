use mongodb::{
    bson::doc,
    options::{IndexOptions, InsertManyOptions},
    results::{CreateIndexResult, InsertManyResult},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TwitchChatMessage {
    pub channel: String,
    pub sender: String,
    pub message: String,
    pub timestamp: mongodb::bson::DateTime,
    pub message_score: f64,
}

pub async fn save_messages(
    db_client: &mongodb::Database,
    messages: impl IntoIterator<Item = TwitchChatMessage>,
) -> Result<InsertManyResult, mongodb::error::Error> {
    let collection = db_client.collection::<TwitchChatMessage>("twitch_messages");

    collection
        .insert_many(
            messages,
            InsertManyOptions::builder().ordered(Some(false)).build(),
        )
        .await
}

/// Have to make a unique compound index since SQS standard queues can have dupplications
pub async fn ensure_message_index_exists(
    db_client: &mongodb::Database,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let collection = db_client.collection::<TwitchChatMessage>("twitch_messages");

    collection
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "channel": 1, "sender": 1, "message": 1, "timestamp": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await
}
