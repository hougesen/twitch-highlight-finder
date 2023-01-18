use database::{
    clips::{get_pending_clip, save_clip_state, save_video_url},
    get_db_client,
};

mod clipping;
mod storage;

const VIDEO_TIME_BUFFER: i64 = 10_000;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let db_client = get_db_client(
        &dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI"),
    )
    .await?
    .database("highlights");

    let s3_client = storage::setup_s3().await;

    if let Some(clip) = get_pending_clip(&db_client).await? {
        let clip_id_str = clip.id.to_string();

        let video_url = clipping::get_platform_url(&clip.vod_id);

        if let Some(download_url) = clipping::get_download_url(&video_url).await {
            let start = ms_to_s(std::cmp::max(0, clip.start_time - VIDEO_TIME_BUFFER));

            let duration = ms_to_s(clip.end_time + VIDEO_TIME_BUFFER) - start;

            let download_result =
                clipping::download_video(download_url.trim(), &clip_id_str, start, duration).await;

            if download_result.is_ok() {
                let uploaded = storage::upload_video(&s3_client, &clip_id_str).await;

                if uploaded.is_ok() {
                    save_video_url(
                        &db_client,
                        clip.id,
                        format!(
                            "https://{}.s3.eu-central-1.amazonaws.com/{}.mp4",
                            storage::S3_BUCKET_NAME,
                            clip_id_str
                        ),
                    )
                    .await?;
                } else {
                    save_clip_state(&db_client, clip.id, "upload-failed").await?;
                }
            }
        } else {
            save_clip_state(&db_client, clip.id, "download-failed").await?;
        }

        clipping::remove_video(&clip_id_str).await.ok();
    }

    Ok(())
}

#[inline]
pub fn ms_to_s(ms: i64) -> i64 {
    ms / 1000
}
