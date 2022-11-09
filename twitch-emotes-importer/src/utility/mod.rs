use crate::twitch::authentication::authenticate_twitch;

pub async fn build_http_client() -> Result<reqwest::Client, reqwest::Error> {
    if let Ok(headers) = authenticate_twitch().await {
        let client_builder = reqwest::ClientBuilder::new().default_headers(headers);

        return client_builder.build();
    }

    // TODO: handle this?
    panic!("Could not authenticate to Twitch")
}
