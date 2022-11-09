use mongodb::bson::doc;
use mongodb::options::{IndexOptions, InsertManyOptions};
use mongodb::results::{CreateIndexResult, InsertManyResult};
use mongodb::{Collection, Database, IndexModel};

use crate::twitch::emotes::TwitchEmote;

pub async fn get_db_client() -> Result<Database, mongodb::error::Error> {
    let db_connection_string =
        dotenv::var("MONGO_CONNECTION_STRING").expect("Missing env MONGO_CONNECTION_STRING");

    let client = mongodb::Client::with_uri_str(db_connection_string).await?;

    Ok(client.database("highlights"))
}

pub async fn ensure_unique_index_exists(
    collection: &Collection<TwitchEmote>,
) -> Result<CreateIndexResult, mongodb::error::Error> {
    let index = IndexModel::builder()
        .keys(doc! {
            "emote_id": 1
        })
        .options(IndexOptions::builder().unique(Some(true)).build())
        .build();

    collection.create_index(index, None).await
}

pub async fn save_emotes(
    emotes: Vec<TwitchEmote>,
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
