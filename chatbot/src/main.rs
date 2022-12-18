use async_channel::unbounded;
use mongodb::bson::DateTime;

use crate::chat::socket_thread;
use crate::db::{get_channel_queue, get_db_client};
use crate::parser::message_parser_thread;

mod chat;
mod db;
mod parser;

#[tokio::main]
async fn main() -> Result<(), tungstenite::Error> {
    println!("Starting Twitch Chat Collector");

    let db_client = get_db_client().await.unwrap();

    let channel_queue = get_channel_queue(&db_client).await;

    let (message_tx, message_rx) = unbounded::<(String, DateTime)>();

    tokio::spawn(async move { socket_thread(channel_queue, message_tx).await });

    message_parser_thread(db_client, message_rx).await
}
