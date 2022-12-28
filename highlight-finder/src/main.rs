use db::twitch_vods::TwitchVodModel;

mod db;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    println!("Hello, world!");

    let db_client = db::get_db_client().await?;

    if let Some(vod) = db::twitch_vods::get_pending_vod(&db_client).await {
        println!("vod: {:#?}", vod);

        analyze_vod(&db_client, vod).await?;
    }

    Ok(())
}

async fn analyze_vod(
    db_client: &mongodb::Database,
    vod: TwitchVodModel,
) -> Result<(), mongodb::error::Error> {
    db::twitch_messages::get_vod_message_scores(
        db_client,
        &vod.channel_name,
        vod.streamed_at,
        vod.ended_at,
    )
    .await?;

    Ok(())
}
