use database::{
    channels::get_channel_ids,
    get_db_client,
    twitch_vods::{save_vods, PartialTwitchVodModel},
};
use mongodb::bson::DateTime;

use crate::twitch::{
    authentication::authenticate,
    build_http_client,
    videos::{calculate_video_duration, get_twitch_videos},
};

mod error;
mod twitch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_client = get_db_client(
        &dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI"),
    )
    .await?
    .database("highlights");

    let channel_queue = get_channel_ids(&db_client).await?;

    if !channel_queue.is_empty() {
        let twitch_client_id = dotenv::var("CLIENT_ID").expect("ERROR: Missing CLIENT_ID env");
        let twitch_client_secret =
            dotenv::var("CLIENT_SECRET").expect("ERROR: Missing CLIENT_SECRET env");

        let twitch_token = authenticate(&twitch_client_id, &twitch_client_secret).await?;

        let http_client = build_http_client(&twitch_client_id, &twitch_token.access_token)?;

        let mut vods: Vec<PartialTwitchVodModel> = Vec::new();

        for channel_id in channel_queue {
            if let Ok(video_response) = get_twitch_videos(&http_client, &channel_id).await {
                if !video_response.data.is_empty() {
                    for video in video_response.data {
                        let video_duration = calculate_video_duration(&video.duration);

                        // TODO: serialize directly into mongodb::bson::DateTime
                        let created_at = DateTime::parse_rfc3339_str(video.created_at)?;

                        vods.push(PartialTwitchVodModel {
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
            save_vods(&db_client, vods).await.ok();
        }
    }

    Ok(())
}
