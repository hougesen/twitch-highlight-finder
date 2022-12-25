#[derive(Debug, serde::Deserialize)]
pub struct TwitchErrorResponse {
    pub status: i32,
    pub error: Option<String>,
    pub message: Option<String>,
}

pub(super) fn unwrap_twitch_error(error: TwitchErrorResponse) -> TwitchError {
    let formatted_message = format!(
        "{} {} {}",
        error.status,
        error.error.unwrap_or_default(),
        error.message.unwrap_or_default()
    );

    match error.status {
        400 => TwitchError::BadRequest(formatted_message),
        401 => TwitchError::Unauthorized(formatted_message),
        403 => TwitchError::Forbidden(formatted_message),
        404 => TwitchError::NotFound(formatted_message),
        409 => TwitchError::Conflict(formatted_message),
        429 => TwitchError::TooManyRequests(formatted_message),
        _ => TwitchError::UnknownError(formatted_message),
    }
}

#[derive(Debug)]
pub enum TwitchError {
    ReqwestError(reqwest::Error),
    MissingClientId,
    MissingClientSecret,
    MissingAccessToken,
    InvalidParameters(String),
    // Twitch related
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    TooManyRequests(String),
    UnknownError(String),
}

impl std::error::Error for TwitchError {}

impl std::fmt::Display for TwitchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwitchError::ReqwestError(s) => s.fmt(f),
            TwitchError::BadRequest(message) => write!(f, "{}", message),
            TwitchError::Unauthorized(message) => write!(f, "{}", message),
            TwitchError::Forbidden(message) => write!(f, "{}", message),
            TwitchError::NotFound(message) => write!(f, "{}", message),
            TwitchError::TooManyRequests(message) => write!(f, "{}", message),
            TwitchError::Conflict(message) => write!(f, "{}", message),
            TwitchError::UnknownError(message) => write!(f, "{}", message),
            TwitchError::MissingClientId => write!(f, "Error: Missing Twitch client id"),
            TwitchError::MissingClientSecret => write!(f, "Error: Missing Twitch client secret"),
            TwitchError::MissingAccessToken => write!(f, "Error: Missing Twitch access token"),
            TwitchError::InvalidParameters(message) => {
                write!(f, "Error: Invalid parameters {}", message)
            }
        }
    }
}
