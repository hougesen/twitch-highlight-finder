mod db;
mod importer;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let mut emotes = importer::fetch_emotes(10000).await;

    println!("len: {}", emotes.len());

    emotes.append(&mut importer::fetch_global_emotes().await);

    db::save_emotes(emotes).await.ok();

    Ok(())
}
