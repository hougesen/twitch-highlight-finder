use async_channel::Receiver;
use aws_sdk_sqs as sqs;
use mongodb::bson::DateTime;

pub async fn setup_sqs_client() -> sqs::Client {
    let config = aws_config::load_from_env().await;

    sqs::Client::new(&config)
}

pub async fn create_queue(
    sqs_client: &sqs::Client,
    queue_name: &str,
) -> Result<sqs::output::CreateQueueOutput, sqs::types::SdkError<sqs::error::CreateQueueError>> {
    sqs_client
        .create_queue()
        .queue_name(queue_name)
        .send()
        .await
}

#[derive(serde::Serialize)]
pub struct QueueMessage {
    pub message: String,
    timestamp: DateTime,
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
        if let Ok((m, t)) = message_rx.recv().await {
            sqs_client
                .send_message()
                .queue_url(queue_url)
                .message_body(
                    serde_json::to_string(&QueueMessage {
                        message: m,
                        timestamp: t,
                    })
                    .unwrap(),
                )
                .send()
                .await?;
        }
    }

    Ok(())
}
