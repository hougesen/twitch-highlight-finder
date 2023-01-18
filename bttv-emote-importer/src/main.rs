use database::{emotes::save_emotes, get_db_client};

mod importer;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let mut emotes = importer::fetch_emotes(10000).await;

    println!("len: {}", emotes.len());

    emotes.append(&mut importer::fetch_global_emotes().await);

    let db_client = get_db_client(
        &dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI"),
    )
    .await?;

    save_emotes(&db_client.database("highlights"), emotes)
        .await
        .ok();

    Ok(())
}
