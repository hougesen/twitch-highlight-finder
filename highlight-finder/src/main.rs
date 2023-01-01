mod db;
mod finder;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let db_client = db::get_db_client().await?;

    let vods = db::twitch_vods::get_all_pendings_vods(&db_client).await;

    for vod in vods {
        let vod_id = vod.id;

        let clips = finder::analyze_vod(&db_client, vod).await?;

        if !clips.is_empty() {
            if let Ok(insert_result) = db::clips::save_clips(&db_client, clips).await {
                let clip_ids = insert_result.inserted_ids;
            }
        }

        db::twitch_vods::mark_as_analyzed(&db_client, vod_id).await?;
    }

    Ok(())
}
