use reqwest::header::{HeaderMap, HeaderValue};

/// NOTE: The response also includes a token TTL & token type.
/// Since this is meant to be a shortlived job it doesn't matter for us.
#[derive(serde::Deserialize)]
struct GetTwitchAccessTokenResponse {
    access_token: String,
}

/// Used for getting headers for authenticating with Twitch's API
pub async fn authenticate_twitch() -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let client_id = dotenv::var("CLIENT_ID").expect("Missing env CLIENT_ID");
    let client_secret = dotenv::var("CLIENT_SECRET").expect("Missing env CLIENT_SECRET");

    let token_url = format!("https://id.twitch.tv/oauth2/token?client_id={client_id}&client_secret={client_secret}&grant_type=client_credentials&state=def");

    let token_req = reqwest::Client::new().post(token_url).send().await?;

    let parsed_token_req = token_req.json::<GetTwitchAccessTokenResponse>().await?;

    let mut headers = HeaderMap::new();

    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer: {}", &parsed_token_req.access_token))?,
    );

    headers.insert("Client-Id", HeaderValue::from_str(&client_id)?);

    Ok(headers)
}
