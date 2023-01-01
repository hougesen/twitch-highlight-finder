use mongodb::{
    bson::doc,
    options::{IndexOptions, InsertManyOptions},
    results::{CreateIndexesResult, InsertManyResult},
    {Collection, Database, IndexModel},
};

#[derive(serde::Serialize)]
pub struct TwitchEmote {
    pub emote_id: String,
    /// The name of the emote. This is the name that viewers type in the chat window to get the emote to appear.
    pub name: String,
    pub channel_id: Option<String>,
}

pub async fn get_db_client() -> Result<Database, mongodb::error::Error> {
    let mongo_uri = dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI");

    let client = mongodb::Client::with_uri_str(mongo_uri).await?;

    Ok(client.database("highlights"))
}

pub async fn ensure_unique_index_exists(
    collection: &Collection<TwitchEmote>,
) -> Result<CreateIndexesResult, mongodb::error::Error> {
    collection
        .create_indexes(
            [
                IndexModel::builder()
                    .keys(doc! { "emote_id": 1 })
                    .options(IndexOptions::builder().unique(Some(true)).build())
                    .build(),
                IndexModel::builder()
                    .keys(doc! { "name": 1 })
                    .options(IndexOptions::builder().unique(Some(true)).build())
                    .build(),
            ],
            None,
        )
        .await
}

pub async fn save_emotes(
    emotes: impl IntoIterator<Item = TwitchEmote>,
) -> Result<InsertManyResult, mongodb::error::Error> {
    let db = get_db_client().await?;

    let collection = db.collection::<TwitchEmote>("emotes");

    ensure_unique_index_exists(&collection).await?;

    collection
        .insert_many(
            emotes,
            InsertManyOptions::builder().ordered(Some(false)).build(),
        )
        .await
}
