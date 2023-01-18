use database::{channels::get_channel_ids, emotes::save_emotes, get_db_client};

use crate::{
    twitch::emotes::{fetch_channel_emotes, fetch_global_emotes},
    utility::build_http_client,
};

mod twitch;
mod utility;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = build_http_client().await?;

    let mut emotes = fetch_global_emotes(&http_client).await.unwrap_or_default();

    let db_client = get_db_client(
        &dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI"),
    )
    .await?
    .database("highlights");

    // NOTE: if this was production code i would most likely implement this using a mqtt queue
    if let Ok(channel_ids) = get_channel_ids(&db_client).await {
        for channel_id in channel_ids {
            if let Ok(mut channel_emotes) = fetch_channel_emotes(&channel_id, &http_client).await {
                emotes.append(&mut channel_emotes);
            }
        }
    }

    println!("emotes len: {}", emotes.len());

    // Handling the error doesn't really matter since we have set insert_many to be unordered
    save_emotes(&db_client, emotes).await.ok();

    Ok(())
}
