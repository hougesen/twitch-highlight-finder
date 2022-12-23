use async_channel::unbounded;
use mongodb::bson::DateTime;

use crate::chat::chat_listener;

mod chat;
mod database;
mod queue;

#[tokio::main]
async fn main() -> Result<(), tungstenite::Error> {
    println!("Starting Twitch Chat Collector");

    let (message_tx, message_rx) = unbounded::<(String, DateTime)>();

    let chat_handle = tokio::spawn(async move { chat_listener(message_tx).await });

    queue::message_queuer(message_rx).await.unwrap();

    chat_handle.abort();

    Ok(())
}
