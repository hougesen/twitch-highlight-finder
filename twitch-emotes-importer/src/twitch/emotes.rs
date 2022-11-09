#[derive(Debug, serde::Deserialize)]
struct TwitchGetGlobalEmotesData {
    id: String,
    name: String,
}

#[derive(serde::Deserialize)]
struct TwitchGetEmotesResponse {
    data: Vec<TwitchGetGlobalEmotesData>,
}

#[derive(Debug, serde::Serialize)]
pub struct TwitchEmote {
    pub id: String,
    /// The name of the emote. This is the name that viewers type in the chat window to get the emote to appear.
    pub name: String,
    pub channel_id: Option<String>,
}

pub async fn fetch_global_emotes(
    http_client: &reqwest::Client,
) -> Result<Vec<TwitchEmote>, Box<dyn std::error::Error>> {
    let url = "https://api.twitch.tv/helix/chat/emotes/global";

    let response = http_client.get(url).send().await?;

    let mut emotes: Vec<TwitchEmote> = Vec::new();

    if let Ok(parsed_response) = response.json::<TwitchGetEmotesResponse>().await {
        if !parsed_response.data.is_empty() {
            for emote in parsed_response.data {
                emotes.push(TwitchEmote {
                    id: emote.id,
                    name: emote.name,
                    channel_id: None,
                })
            }
        }
    }

    Ok(emotes)
}

pub async fn fetch_channel_emotes(
    channel_id: String,
    http_client: &reqwest::Client,
) -> Result<Vec<TwitchEmote>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.twitch.tv/helix/chat/emotes?broadcaster_id={}",
        &channel_id
    );

    let response = http_client.get(url).send().await?;

    let mut emotes = Vec::new();

    if let Ok(parsed_response) = response.json::<TwitchGetEmotesResponse>().await {
        if !parsed_response.data.is_empty() {
            for emote in parsed_response.data {
                emotes.push(TwitchEmote {
                    id: emote.id,
                    name: emote.name,
                    channel_id: Some(channel_id.clone()),
                })
            }
        }
    }

    Ok(emotes)
}
