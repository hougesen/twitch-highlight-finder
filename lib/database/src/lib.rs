pub mod channels;
pub mod clips;
pub mod emotes;
pub mod twitch_messages;
pub mod twitch_vods;

pub async fn get_db_client(
    mongo_connection_uri: &str,
) -> Result<mongodb::Client, mongodb::error::Error> {
    mongodb::Client::with_uri_str(mongo_connection_uri).await
}
