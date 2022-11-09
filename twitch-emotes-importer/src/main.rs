use dotenv::dotenv;
use twitch::{authentication::authenticate_twitch, emotes::fetch_global_emotes};

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

    let global_emotes = fetch_global_emotes(&http_client).await?;

    Ok(())
}
