use crate::error::TwitchError;

use super::unwrap_twitch_response;

#[derive(Debug, serde::Deserialize)]
pub struct GetTwitchAccessTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
}

pub async fn authenticate(
    client_id: &str,
    client_secret: &str,
) -> Result<GetTwitchAccessTokenResponse, TwitchError> {
    if client_id.is_empty() {
        return Err(TwitchError::MissingClientId);
    }

    if client_secret.is_empty() {
        return Err(TwitchError::MissingClientSecret);
    }

    let auth_uri = format!(
        "https://id.twitch.tv/oauth2/token?client_id={client_id}&client_secret={client_secret}&grant_type=client_credentials&state=def"
    );

    let request = reqwest::Client::new().post(auth_uri).send().await;

    unwrap_twitch_response(request).await
}

#[cfg(test)]
mod authenticate {
    use crate::{error::TwitchError, twitch::authentication::authenticate};

    #[tokio::test]
    async fn valid_token() {
        dotenv::dotenv().ok();

        let r = authenticate(
            &dotenv::var("CLIENT_ID").unwrap(),
            &dotenv::var("CLIENT_SECRET").unwrap(),
        )
        .await;

        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn missing_client_id() {
        dotenv::dotenv().ok();

        let r = authenticate(&String::new(), &dotenv::var("CLIENT_SECRET").unwrap()).await;

        assert!(r.is_err());

        assert!(matches!(r.unwrap_err(), TwitchError::MissingClientId));
    }

    #[tokio::test]
    async fn missing_client_secret() {
        dotenv::dotenv().ok();

        let r = authenticate(&dotenv::var("CLIENT_ID").unwrap(), &String::new()).await;

        assert!(r.is_err());

        assert!(matches!(r.unwrap_err(), TwitchError::MissingClientSecret));
    }

    #[tokio::test]
    async fn invalid_client_id() {
        dotenv::dotenv().ok();

        let r = authenticate(
            &format!("{}-invalid", dotenv::var("CLIENT_ID").unwrap()),
            &dotenv::var("CLIENT_SECRET").unwrap(),
        )
        .await;

        assert!(r.is_err());

        assert!(matches!(r.unwrap_err(), TwitchError::BadRequest(..)));
    }

    #[tokio::test]
    async fn invalid_client_secret() {
        dotenv::dotenv().ok();

        let r = authenticate(
            &dotenv::var("CLIENT_ID").unwrap(),
            &format!("{}-invalid", dotenv::var("CLIENT_SECRET").unwrap()),
        )
        .await;

        assert!(r.is_err());

        assert!(matches!(r.unwrap_err(), TwitchError::Forbidden(..)));
    }
}
