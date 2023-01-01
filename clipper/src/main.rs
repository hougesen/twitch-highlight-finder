mod db;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    println!("Hello, world!");

    let db_client = db::get_db_client().await?;

    if let Ok(res) = db::clips::get_pending_clip(&db_client).await {
        if let Some(clip) = res {
        }
    }

    Ok(())
}

