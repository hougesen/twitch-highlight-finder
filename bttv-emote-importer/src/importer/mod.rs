use crate::db::TwitchEmote;

#[derive(serde::Deserialize)]
struct BetterTTVEmote {
    id: String,
    code: String,
}

#[derive(serde::Deserialize)]
struct BetterTTVResponse {
    emote: BetterTTVEmote,
}

pub async fn fetch_global_emotes() -> Vec<TwitchEmote> {
    let mut emotes = Vec::new();

    let http_client = reqwest::Client::new();

    if let Ok(response) = http_client
        .get("https://api.betterttv.net/3/cached/emotes/global")
        .send()
        .await
    {
        if let Ok(parsed_response) = response.json::<Vec<BetterTTVEmote>>().await {
            for emote in parsed_response {
                emotes.push(transform_emote(emote))
            }
        }
    }

    emotes
}

pub async fn fetch_emotes(max: usize) -> Vec<TwitchEmote> {
    let mut emotes = Vec::new();

    let http_client = reqwest::Client::new();

    while emotes.len() < max {
        let url = format!(
            "https://api.betterttv.net/3/emotes/shared/top?limit=100&offset={}",
            emotes.len()
        );

        if let Ok(response) = http_client.get(url).send().await {
            match response.json::<Vec<BetterTTVResponse>>().await {
                Ok(parsed_response) => {
                    if parsed_response.is_empty() {
                        break;
                    }

                    for element in parsed_response {
                        emotes.push(transform_emote(element.emote))
                    }
                }
                Err(_) => break,
            }
        } else {
            break;
        }
    }

    emotes
}

#[inline]
fn transform_emote(emote: BetterTTVEmote) -> TwitchEmote {
    TwitchEmote {
        emote_id: emote.id,
        name: emote.code,
        channel_id: None,
    }
}
