use chat::socket_thread;
use dotenv::dotenv;
use queue::Queue;

mod chat;
mod queue;

fn main() -> Result<(), tungstenite::Error> {
    println!("Starting Twitch Chat Collector");

    dotenv().ok();

    let channel_join_queue = Queue::from(vec!["ESL_CSGO".to_string()]);

    socket_thread(channel_join_queue)?;

    Ok(())
}
