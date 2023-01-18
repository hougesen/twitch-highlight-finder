use async_channel::Receiver;
use aws_sdk_sqs::{
    error::{CreateQueueError, SendMessageError},
    output::{CreateQueueOutput, SendMessageOutput},
    types::SdkError,
};
use mongodb::bson::DateTime;

pub mod channel;

pub async fn setup_sqs_client() -> aws_sdk_sqs::Client {
    let config = aws_config::load_from_env().await;

    aws_sdk_sqs::Client::new(&config)
}

pub async fn create_queue(
    sqs_client: &aws_sdk_sqs::Client,
    queue_name: &str,
) -> Result<CreateQueueOutput, SdkError<CreateQueueError>> {
    sqs_client
        .create_queue()
        .queue_name(queue_name)
        .send()
        .await
}

#[derive(serde::Serialize)]
pub struct QueueMessage {
    pub message: String,
    pub timestamp: DateTime,
}

pub async fn message_queuer(
    message_rx: Receiver<(String, mongodb::bson::DateTime)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let sqs_client = setup_sqs_client().await;

    // Call on startup to make sure our queue exists.
    let queue = create_queue(&sqs_client, "unparsed-messages")
        .await
        .unwrap();

    let queue_url = queue.queue_url().unwrap();

    while !message_rx.is_closed() {
        if let Ok((original_message, timestamp)) = message_rx.recv().await {
            let trimmed_message = original_message.trim();

            for message in trimmed_message.split("\r\n").collect::<Vec<&str>>() {
                if message.contains("PRIVMSG") {
                    queue_message(&sqs_client, queue_url, message.trim(), timestamp)
                        .await
                        .ok();
                } else {
                    eprintln!("UNKNOWN MESSAGE: {message}");
                }
            }
        }
    }

    eprintln!("outside messageq_queuer");

    Ok(())
}

#[inline]
async fn queue_message(
    sqs_client: &aws_sdk_sqs::Client,
    queue_url: &str,
    message: &str,
    timestamp: DateTime,
) -> Result<SendMessageOutput, SdkError<SendMessageError>> {
    sqs_client
        .send_message()
        .queue_url(queue_url)
        .message_body(
            serde_json::to_string(&QueueMessage {
                message: message.to_string(),
                timestamp,
            })
            .unwrap(),
        )
        .send()
        .await
}
