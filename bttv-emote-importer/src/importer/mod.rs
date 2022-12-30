use crate::db::TwitchEmote;

#[derive(Debug, serde::Deserialize)]
struct BetterTTVEmote {
    id: String,
    code: String,
}

#[derive(Debug, serde::Deserialize)]
struct BetterTTVResponse {
    emote: BetterTTVEmote,
}

pub async fn fetch_emotes(max: usize) -> Vec<TwitchEmote> {
    let mut emotes = Vec::new();

    let http_client = reqwest::Client::new();

    while emotes.len() < max {
        let url = format!(
            "https://api.betterttv.net/3/emotes/shared/top?offset={}&limit=100",
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
