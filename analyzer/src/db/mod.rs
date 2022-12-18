use dashmap::DashMap;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
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

#[derive(serde::Deserialize)]
pub struct TwitchChatMessage {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub channel: String,
    pub sender: String,
    pub message: String,
    pub timestamp: mongodb::bson::DateTime,
    pub message_score: Option<f32>,
}

#[derive(serde::Deserialize)]
pub struct FoundMessage {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub message: String,
}

pub async fn get_pending_chat_messages(db_client: &Database) -> Vec<FoundMessage> {
    let collection = db_client.collection::<TwitchChatMessage>("twitch_messages");

    let cursor = collection
        .aggregate(
            vec![
                doc! {
                    "$match": {
                        "message_score": {
                            "$exists": false
                        }
                    }
                },
                doc! {
                    "$project": {
                        "_id": 1,
                        "message": 1
                    }
                },
            ],
            None,
        )
        .await;

    if let Ok(cursor) = cursor {
        let messages = cursor
            .with_type::<FoundMessage>()
            .try_collect::<Vec<FoundMessage>>()
            .await;

        if let Ok(messages) = messages {
            return messages;
        }
    }

    vec![]
}

pub async fn save_message_score(
    db_client: Database,
    message_id: ObjectId,
    message_score: u8,
) -> Result<mongodb::results::UpdateResult, mongodb::error::Error> {
    let collection = db_client.collection::<TwitchChatMessage>("twitch_messages");

    collection
        .update_one(
            doc! {
                "_id": message_id
            },
            doc! {
                "$set": {
                    "message_score": f64::from(message_score) / 100.0
                }
            },
            None,
        )
        .await
}
