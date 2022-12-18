use crossbeam_channel::unbounded;
use dotenv::dotenv;
use mongodb::bson::DateTime;

use crate::chat::socket_thread;
use crate::db::{get_channel_queue, get_db_client};
use crate::parser::message_parser_thread;

mod chat;
mod db;
mod parser;

fn main() -> Result<(), tungstenite::Error> {
    println!("Starting Twitch Chat Collector");

    dotenv().ok();

    let db_client = get_db_client();

    let channel_queue = get_channel_queue(&db_client);

    let (message_tx, message_rx) = unbounded::<(String, DateTime)>();

    std::thread::spawn(|| socket_thread(channel_queue, message_tx));

    message_parser_thread(db_client, message_rx);
}
