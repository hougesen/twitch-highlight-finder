use dotenv::dotenv;
use twitch::{
    authentication::authenticate_twitch,
    emotes::{fetch_channel_emotes, fetch_global_emotes, TwitchEmote},
};

mod twitch;

async fn build_http_client() -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let headers = authenticate_twitch().await?;

    let client_builder = reqwest::ClientBuilder::new().default_headers(headers);

    Ok(client_builder.build()?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let http_client = build_http_client().await?;

    let mut found_emotes: Vec<TwitchEmote> = Vec::new();

    if let Ok(mut global_emotes) = fetch_global_emotes(&http_client).await {
        found_emotes.append(&mut global_emotes);
    }

    // NOTE: if this was production code i would most likely implement this using a mqtt queue
    let channel_ids = vec!["31239503".to_string(), "35936871".to_string()];

    for channel_id in channel_ids {
        if let Ok(mut channel_emotes) = fetch_channel_emotes(channel_id, &http_client).await {
            found_emotes.append(&mut channel_emotes);
        }
    }

    Ok(())
}
