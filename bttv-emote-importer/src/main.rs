use mongodb::results::InsertManyResult;

mod db;
mod importer;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    let emotes = importer::fetch_emotes(10000).await;

    println!("len: {}", emotes.len());

    db::save_emotes(emotes).await.ok();

    Ok(())
}
