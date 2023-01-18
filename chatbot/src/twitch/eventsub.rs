use crate::{database::get_db_channels, twitch::authentication::authenticate_twitch};

#[derive(serde::Serialize)]
struct Condition {
    broadcaster_user_id: String,
}

impl Condition {
    fn new(broadcaster_user_id: &str) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.to_string(),
        }
    }
}

#[derive(serde::Serialize)]
struct Transport {
    method: String,
    callback: String,
    secret: String,
}

impl Transport {
    fn new(callback: &str, secret: &str) -> Self {
        Self {
            method: "webhook".to_string(),
            callback: callback.to_string(),
            secret: secret.to_string(),
        }
    }
}

#[derive(serde::Serialize)]
struct TwitchEventSubBody {
    r#type: String,
    version: String,
    condition: Condition,
    transport: Transport,
}

impl TwitchEventSubBody {
    fn new(r#type: &str, channel: &str, callback: &str, secret: &str) -> Self {
        Self {
            r#type: r#type.to_string(),
            version: "1".to_string(),
            condition: Condition::new(channel),
            transport: Transport::new(callback, secret),
        }
    }
}

pub async fn subscribe_to_channels() -> Result<(), Box<dyn std::error::Error>> {
    let channels = get_db_channels().await;

    if !channels.is_empty() {
        let http_client = reqwest::ClientBuilder::new()
            .default_headers(
                authenticate_twitch()
                    .await
                    .expect("error authenticating twitch"),
            )
            .build()
            .expect("error creating http client ");

        let callback = dotenv::var("WEBHOOK_CALLBACK")?;
        let secret = dotenv::var("WEBHOOK_SECRET")?;

        let stream_online = "stream.online";
        let stream_offline = "stream.offline";

        let mut body = TwitchEventSubBody::new(stream_online, "", &callback, &secret);

        for channel in channels {
            println!("Sending subscribtion to: {channel}");

            body.r#type = stream_online.to_string();

            body.condition.broadcaster_user_id = channel;

            http_client
                .post("https://api.twitch.tv/helix/eventsub/subscriptions")
                .json(&body)
                .send()
                .await
                .ok();

            body.r#type = stream_offline.to_string();

            http_client
                .post("https://api.twitch.tv/helix/eventsub/subscriptions")
                .json(&body)
                .send()
                .await
                .ok();
        }
    }

    Ok(())
}
