use dashmap::DashMap;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::{IndexOptions, InsertManyOptions},
    results::{CreateIndexesResult, InsertManyResult},
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TwitchEmote {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub emote_id: String,
    /// The name of the emote. This is the name that viewers type in the chat window to get the emote to appear.
    pub name: String,
    pub channel_id: Option<String>,
    pub score: Option<u8>,
}

pub async fn ensure_unique_index_exists(
    collection: &mongodb::Collection<TwitchEmote>,
) -> Result<CreateIndexesResult, mongodb::error::Error> {
    collection
        .create_indexes(
            [
                mongodb::IndexModel::builder()
                    .keys(doc! { "emote_id": 1 })
                    .options(IndexOptions::builder().unique(Some(true)).build())
                    .build(),
                mongodb::IndexModel::builder()
                    .keys(doc! { "name": 1 })
                    .options(IndexOptions::builder().unique(Some(true)).build())
                    .build(),
            ],
            None,
        )
        .await
}

pub async fn save_emotes<T: for<'a> serde::Serialize>(
    db_client: &mongodb::Database,
    emotes: impl IntoIterator<Item = T>,
) -> Result<InsertManyResult, mongodb::error::Error> {
    let collection = db_client.collection::<T>("emotes");

    collection
        .insert_many(
            emotes,
            InsertManyOptions::builder().ordered(Some(false)).build(),
        )
        .await
}

pub async fn get_emote_scores(db_client: &mongodb::Database) -> DashMap<String, u8> {
    let collection = db_client.collection::<TwitchEmote>("emotes");

    let mut emote_scores: DashMap<String, u8> = DashMap::new();

    // TODO: convert to aggregate to reduce data size?
    if let Ok(cursor) = collection.find(None, None).await {
        if let Ok(emotes) = cursor.try_collect::<Vec<TwitchEmote>>().await {
            emote_scores.try_reserve(emotes.len()).ok();

            for emote in emotes {
                emote_scores.insert(emote.name, emote.score.unwrap_or(1));
            }
        };
    }

    emote_scores.shrink_to_fit();

    emote_scores
}
