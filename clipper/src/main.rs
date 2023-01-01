mod clipping;
mod db;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    println!("Hello, world!");

    let db_client = db::get_db_client().await?;

    if let Ok(res) = db::clips::get_pending_clip(&db_client).await {
        if let Some(clip) = res {
            let video_url = clipping::get_platform_url(&clip.vod_id);

            if let Some(download_url) = clipping::get_download_url(&video_url).await {}
        }
    }

    Ok(())
}

