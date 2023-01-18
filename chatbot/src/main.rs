use async_channel::unbounded;
use mongodb::bson::DateTime;

use crate::{chat::chat_listener, twitch::eventsub::subscribe_to_channels};

mod chat;
mod database;
mod queue;
mod twitch;

#[tokio::main]
async fn main() -> Result<(), tungstenite::Error> {
    println!("Starting Twitch Chat Collector");

    subscribe_to_channels().await.unwrap();

    let (message_tx, message_rx) = unbounded::<(String, DateTime)>();

    let chat_handle = tokio::spawn(async move { chat_listener(message_tx).await });

    queue::message_queuer(message_rx).await.unwrap();

    chat_handle.abort();

    Ok(())
}
