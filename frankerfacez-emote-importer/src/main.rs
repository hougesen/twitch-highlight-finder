mod db;
mod importer;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let emotes = importer::fetch_emotes(25).await;

    println!("Found {} emotes", emotes.len());

    db::save_emotes(emotes).await.ok();

    Ok(())
}
