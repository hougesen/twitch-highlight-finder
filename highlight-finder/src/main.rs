mod db;
mod finder;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    println!("Hello, world!");

    let db_client = db::get_db_client().await?;

    let vods = db::twitch_vods::get_all_pendings_vods(&db_client).await;

    for vod in vods {
        println!("vod: {:#?}", vod);

        finder::analyze_vod(&db_client, vod).await?;
    }

    /*
    if let Some(vod) = db::twitch_vods::get_pending_vod(&db_client).await {
        println!("vod: {:#?}", vod);

        finder::analyze_vod(&db_client, vod).await?;
    } */

    Ok(())
}
