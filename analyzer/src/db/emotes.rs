use dashmap::DashMap;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};

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

pub async fn get_emote_scores(db_client: &mongodb::Database) -> DashMap<String, u8> {
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
