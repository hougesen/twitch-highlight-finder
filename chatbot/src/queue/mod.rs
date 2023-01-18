use async_channel::Receiver;
use mongodb::bson::DateTime;
use queue::Queue;

pub mod channel;

#[derive(serde::Serialize)]
pub struct QueueMessage {
    pub message: String,
    pub timestamp: DateTime,
}

pub async fn message_queuer(
    message_rx: Receiver<(String, mongodb::bson::DateTime)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut message_queue = Queue::new(None).await;

    let create_queue_output = message_queue.create_queue("unparsed-messages").await?;

    message_queue.set_queue_url(create_queue_output.queue_url().unwrap());

    while !message_rx.is_closed() {
        if let Ok((original_message, timestamp)) = message_rx.recv().await {
            let trimmed_message = original_message.trim();

            for message in trimmed_message.split("\r\n").collect::<Vec<&str>>() {
                if message.contains("PRIVMSG") {
                    if let Ok(m) = serde_json::to_string(&QueueMessage {
                        message: message.to_string(),
                        timestamp,
                    }) {
                        message_queue.queue_message(m).await.ok();
                    }
                } else {
                    eprintln!("UNKNOWN MESSAGE: {message}");
                }
            }
        }
    }

    eprintln!("outside messageq_queuer");

    Ok(())
}
