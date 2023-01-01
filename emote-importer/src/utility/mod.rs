use crate::twitch::authentication::authenticate_twitch;

pub async fn build_http_client() -> Result<reqwest::Client, reqwest::Error> {
    reqwest::ClientBuilder::new()
        .default_headers(authenticate_twitch().await?)
        .build()
}
