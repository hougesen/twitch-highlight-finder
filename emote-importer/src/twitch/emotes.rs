use database::emotes::PartialTwitchEmote;

#[derive(Debug, serde::Deserialize)]
struct TwitchGetGlobalEmotesData {
    id: String,
    name: String,
}

#[derive(serde::Deserialize)]
struct TwitchGetEmotesResponse {
    data: Vec<TwitchGetGlobalEmotesData>,
}

pub async fn fetch_global_emotes(
    http_client: &reqwest::Client,
) -> Result<Vec<PartialTwitchEmote>, reqwest::Error> {
    let url = "https://api.twitch.tv/helix/chat/emotes/global";

    let response = http_client.get(url).send().await?;

    let mut emotes: Vec<PartialTwitchEmote> = Vec::new();

    if let Ok(parsed_response) = response.json::<TwitchGetEmotesResponse>().await {
        if !parsed_response.data.is_empty() {
            for emote in parsed_response.data {
                emotes.push(PartialTwitchEmote {
                    emote_id: emote.id,
                    name: emote.name,
                    channel_id: None,
                })
            }
        }
    }

    Ok(emotes)
}

pub async fn fetch_channel_emotes(
    channel_id: &str,
    http_client: &reqwest::Client,
) -> Result<Vec<PartialTwitchEmote>, reqwest::Error> {
    let url = format!("https://api.twitch.tv/helix/chat/emotes?broadcaster_id={channel_id}");

    let response = http_client.get(url).send().await?;

    let mut emotes = Vec::new();

    if let Ok(parsed_response) = response.json::<TwitchGetEmotesResponse>().await {
        if !parsed_response.data.is_empty() {
            for emote in parsed_response.data {
                emotes.push(PartialTwitchEmote {
                    emote_id: emote.id,
                    name: emote.name,
                    channel_id: Some(channel_id.to_string()),
                })
            }
        }
    }

    Ok(emotes)
}
