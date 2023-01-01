mod clipping;
mod db;

const VIDEO_TIME_BUFFER: i64 = 10_000;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    println!("Hello, world!");

    let db_client = db::get_db_client().await?;

    if let Ok(res) = db::clips::get_pending_clip(&db_client).await {
        if let Some(clip) = res {
            let video_url = clipping::get_platform_url(&clip.vod_id);

            if let Some(download_url) = clipping::get_download_url(&video_url).await {
                let start = ms_to_s(std::cmp::max(0, clip.start_time - VIDEO_TIME_BUFFER));

                let duration = ms_to_s(clip.end_time + VIDEO_TIME_BUFFER) - start;

                let download_result = clipping::download_video(
                    &download_url.trim(),
                    &clip.id.to_string(),
                    start,
                    duration,
                )
                .await;
            }
        }
    }

    Ok(())
}

#[inline]
pub fn ms_to_s(ms: i64) -> i64 {
    ms / 1000
}
