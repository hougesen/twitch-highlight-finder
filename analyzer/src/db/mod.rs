pub mod emotes;
pub mod messages;

pub async fn get_db_client() -> Result<mongodb::Database, mongodb::error::Error> {
    let mongo_uri = dotenv::var("MONGO_CONNECTION_URI").expect("Missing env MONGO_CONNECTION_URI");

    let client = mongodb::Client::with_uri_str(mongo_uri).await?;

    Ok(client.database("highlights"))
}
