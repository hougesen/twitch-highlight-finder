use db::{fetch_channels, save_emotes};
use twitch::emotes::{fetch_channel_emotes, fetch_global_emotes};
use utility::build_http_client;

mod db;
mod twitch;
mod utility;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let http_client = build_http_client().await?;

    let mut emotes = fetch_global_emotes(&http_client).await.unwrap_or_default();

    // NOTE: if this was production code i would most likely implement this using a mqtt queue
    if let Ok(channel_ids) = fetch_channels().await {
        for channel_id in channel_ids {
            if let Ok(mut channel_emotes) = fetch_channel_emotes(&channel_id, &http_client).await {
                emotes.append(&mut channel_emotes);
            }
        }
    }

    println!("emotes len: {}", emotes.len());

    // Handling the error doesn't really matter since we have set insert_many to be unordered
    save_emotes(emotes).await.ok();

    Ok(())
}
