use mongodb::bson::DateTime;

use crate::{
    db::twitch_vods::TwitchVodModel,
    twitch::{
        authentication::authenticate,
        build_http_client,
        videos::{calculate_video_duration, get_twitch_videos},
    },
};

pub mod db;
pub mod error;
pub mod twitch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let db_client = db::get_db_client().await?;

    let channel_queue = db::channels::fetch_channels(&db_client).await?;

    if !channel_queue.is_empty() {
        let twitch_client_id = dotenv::var("CLIENT_ID").expect("ERROR: Missing CLIENT_ID env");
        let twitch_client_secret =
            dotenv::var("CLIENT_SECRET").expect("ERROR: Missing CLIENT_SECRET env");

        let twitch_token = authenticate(&twitch_client_id, &twitch_client_secret).await?;

        let http_client = build_http_client(&twitch_client_id, &twitch_token.access_token)?;

        let mut vods: Vec<TwitchVodModel> = Vec::new();

        for channel_id in channel_queue {
            if let Ok(video_response) = get_twitch_videos(&http_client, channel_id).await {
                if !video_response.data.is_empty() {
                    for video in video_response.data {
                        let video_duration = calculate_video_duration(&video.duration);

                        // TODO: serialize directly into mongodb::bson::DateTime
                        let created_at = DateTime::parse_rfc3339_str(video.created_at).unwrap();

                        vods.push(TwitchVodModel {
                            vod_id: video.id,
                            user_id: video.user_id,
                            language: video.language,
                            stream_id: video.stream_id,
                            channel_name: video.user_login,
                            title: video.title,
                            url: video.url,
                            streamed_at: created_at,
                            ended_at: DateTime::from_millis(
                                created_at.timestamp_millis() + (i64::from(video_duration) * 1000),
                            ),
                            video_duration,
                            analyzed: false,
                        })
                    }
                }
            }
        }
        println!("vods: {}", vods.len());

        if !vods.is_empty() {
            db::twitch_vods::save_vods(&db_client, vods).await?;
        }
    }

    Ok(())
}
