use dashmap::DashMap;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{CreateIndexOptions, InsertManyOptions},
    Database, IndexModel,
};

pub async fn get_db_client() -> Result<Database, mongodb::error::Error> {
    let mongo_uri = dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI");

    let client = mongodb::Client::with_uri_str(mongo_uri).await?;

    Ok(client.database("highlights"))
}

#[derive(Debug, serde::Deserialize)]
pub struct TwitchEmote {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    pub emote_id: String,
    /// The name of the emote. This is the name that viewers type in the chat window to get the emote to appear.
    pub name: String,
    pub channel_id: Option<String>,
    pub score: Option<u8>,
}

pub async fn get_emote_scores(db_client: &Database) -> DashMap<String, u8> {
    let collection = db_client.collection::<TwitchEmote>("emotes");

    let emote_scores: DashMap<String, u8> = DashMap::new();

    // TODO: convert to aggregate to reduce data size?
    if let Ok(cursor) = collection.find(None, None).await {
        if let Ok(emotes) = cursor.try_collect::<Vec<TwitchEmote>>().await {
            for emote in emotes {
                emote_scores.insert(emote.name, emote.score.unwrap_or(1));
            }
        };
    }

    emote_scores
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TwitchChatMessage {
    pub channel: String,
    pub sender: String,
    pub message: String,
    pub timestamp: mongodb::bson::DateTime,
    pub message_score: f64,
}

#[derive(serde::Deserialize)]
pub struct FoundMessage {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub message: String,
}

pub async fn save_message_batch(
    db_client: &Database,
    messages: Vec<TwitchChatMessage>,
) -> Result<mongodb::results::InsertManyResult, mongodb::error::Error> {
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
    db_client: &Database,
) -> Result<mongodb::results::CreateIndexResult, mongodb::error::Error> {
    let collection = db_client.collection::<TwitchChatMessage>("twitch_messages");

    collection
        .create_index(
            IndexModel::builder()
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
