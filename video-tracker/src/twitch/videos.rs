use std::collections::HashMap;

use crate::error::TwitchError;

use super::unwrap_twitch_response;

#[derive(serde::Deserialize)]
pub struct TwitchGetVideosData {
    /// vod_id
    pub id: String,
    pub stream_id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub title: String,
    pub description: String,
    pub created_at: String,   // mongodb::bson::DateTime,
    pub published_at: String, // mongodb::bson::DateTime,
    pub url: String,
    pub thumbnail_url: String,
    pub viewable: String,
    pub view_count: u32,
    pub language: String,
    pub r#type: String,
    /// "3h10m58s",
    pub duration: String,
    pub muted_segments: Option<Vec<HashMap<String, i32>>>,
}

#[derive(serde::Deserialize)]
pub struct TwitchGetVideosResponse {
    pub data: Vec<TwitchGetVideosData>,
}

pub async fn get_twitch_videos(
    http_client: &reqwest::Client,
    user_id: &str,
) -> Result<TwitchGetVideosResponse, TwitchError> {
    let url = format!("https://api.twitch.tv/helix/videos?first=20&type=archive&user_id={user_id}");

    let response = http_client.get(url).send().await;

    unwrap_twitch_response::<TwitchGetVideosResponse>(response).await
}

/// 3h10m58s
pub fn calculate_video_duration(duration: &str) -> u32 {
    let mut seconds = 0;
    let mut minutes = 0;
    let mut hours = 0;
    let mut days = 0;

    let mut current_value = String::new();

    for c in duration.chars() {
        if c.is_ascii_digit() {
            current_value.push(c);
        } else {
            match c {
                's' => {
                    seconds = current_value.parse::<u32>().unwrap_or(0);
                }
                'm' => {
                    minutes = current_value.parse::<u32>().unwrap_or(0);
                }
                'h' => {
                    hours = current_value.parse::<u32>().unwrap_or(0);
                }
                'd' => {
                    days = current_value.parse::<u32>().unwrap_or(0);
                }
                _ => eprintln!("calculate_video_duration unknown char: {c} {current_value}"),
            }

            current_value = String::new();
        }
    }

    seconds + (60 * (minutes + (60 * (hours + (days * 24)))))
}

#[cfg(test)]
mod tests {
    use super::calculate_video_duration;

    #[test]
    fn test_calculalte_video_duration() {
        assert_eq!(0, calculate_video_duration("0s"));
        assert_eq!(5, calculate_video_duration("5s"));

        assert_eq!(15, calculate_video_duration("15s"));

        assert_eq!(59, calculate_video_duration("59s"));

        assert_eq!(60, calculate_video_duration("1m"));
        assert_eq!(60, calculate_video_duration("1m0s"));
        assert_eq!(60, calculate_video_duration("0h1m0s"));
        assert_eq!(84, calculate_video_duration("1m24s"));

        assert_eq!(3600, calculate_video_duration("1h"));
        assert_eq!(3600, calculate_video_duration("1h0m0s"));
        assert_eq!(7284, calculate_video_duration("2h1m24s"));

        assert_eq!(7284, calculate_video_duration("0d2h1m24s"));
        assert_eq!(86400, calculate_video_duration("1d0h0m0s"));
        assert_eq!(90000, calculate_video_duration("1d1h0m0s"));
        assert_eq!(90060, calculate_video_duration("1d1h1m0s"));
        assert_eq!(90061, calculate_video_duration("1d1h1m1s"));
    }
}
