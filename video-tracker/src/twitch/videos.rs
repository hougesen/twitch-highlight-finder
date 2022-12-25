use std::collections::HashMap;

use super::unwrap_twitch_response;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TwitchGetVideosData {
    /// vod_id
    id: String,
    stream_id: String,
    user_id: String,
    user_login: String,
    user_name: String,
    title: String,
    description: String,
    created_at: mongodb::bson::DateTime,
    published_at: mongodb::bson::DateTime,
    url: String,
    thumbnail_url: String,
    viewable: String,
    view_count: u32,
    language: Option<String>,
    r#type: String,
    /// "3h10m58s",
    duration: String,
    muted_segments: Option<Vec<HashMap<String, i32>>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TwitchGetVideos {}

pub async fn get_twitch_videos(
    http_client: &reqwest::Client,
    user_id: String,
) -> Result<Vec<TwitchGetVideos>, reqwest::Error> {
    let url = format!("https://api.twitch.tv/helix/videos?user_id={user_id}&first=20&type=archive");

    let response = http_client.get(url).send().await;

    unwrap_twitch_response(response).await
}
