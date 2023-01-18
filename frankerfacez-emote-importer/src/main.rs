use database::emotes::save_emotes;

mod importer;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let emotes = importer::fetch_emotes(25).await;

    println!("Found {} emotes", emotes.len());

    let db_client = database::get_db_client(
        &dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI"),
    )
    .await?
    .database("highlights");

    save_emotes(&db_client, emotes).await.ok();

    Ok(())
}
