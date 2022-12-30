use crate::db::TwitchEmote;

#[derive(Debug, serde::Deserialize)]
struct FrankerFaceZEmote {
    id: u64,
    name: String,
}

#[derive(Debug, serde::Deserialize)]
struct FrankerFaceZResponse {
    emoticons: Vec<FrankerFaceZEmote>,
}

pub async fn fetch_emotes(page_count: u16) -> Vec<TwitchEmote> {
    let mut emotes = Vec::new();

    let http_client = reqwest::Client::new();
    let mut page = 1;

    while page < page_count {
        let url = format!(
            "https://api.frankerfacez.com/v1/emotes?sort=count-desc&per_page=200&page={}",
            page
        );

        if let Ok(response) = http_client.get(url).send().await {
            if let Ok(parsed_response) = response.json::<FrankerFaceZResponse>().await {
                if parsed_response.emoticons.is_empty() {
                    break;
                }

                for emote in parsed_response.emoticons {
                    emotes.push(transform_emote(emote))
                }
            }
        }

        page += 1;
    }

    emotes
}

#[inline]
fn transform_emote(emote: FrankerFaceZEmote) -> TwitchEmote {
    TwitchEmote {
        emote_id: format!("frankerfacez-{}", emote.id),
        name: emote.name,
        channel_id: None,
    }
}
