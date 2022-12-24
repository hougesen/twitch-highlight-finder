use aws_sdk_sqs as sqs;
use mongodb::bson::DateTime;
use sqs::{
    error::{CreateQueueError, ReceiveMessageError},
    model::QueueAttributeName,
    output::{CreateQueueOutput, ReceiveMessageOutput},
    types::SdkError,
};

#[derive(Debug, serde::Deserialize)]
pub struct QueueMessage {
    pub message: String,
    pub timestamp: DateTime,
}

async fn setup_sqs_client() -> sqs::Client {
    let config = aws_config::load_from_env().await;

    sqs::Client::new(&config)
}

pub struct Queue {
    sqs_client: sqs::Client,
    queue_url: String,
}

impl Queue {
    pub async fn new(queue_url: Option<String>) -> Self {
        let sqs_client = setup_sqs_client().await;

        Queue {
            sqs_client,
            queue_url: queue_url.unwrap_or_default(),
        }
    }

    pub async fn get_message_batch(
        &self,
        max_messages: Option<i32>,
    ) -> Result<Vec<QueueMessage>, Box<dyn std::error::Error>> {
        let queue_output = self.read_queue(max_messages).await;

        Ok(self.extract_queue_messages(queue_output))
    }

    pub async fn create_queue(
        &self,
        queue_name: &str,
    ) -> Result<CreateQueueOutput, SdkError<CreateQueueError>> {
        self.sqs_client
            .create_queue()
            .queue_name(queue_name)
            .send()
            .await
    }

    pub fn set_queue_url<S: Into<String>>(&mut self, queue_url: S) {
        self.queue_url = queue_url.into()
    }

    async fn read_queue(
        &self,
        max_messages: Option<i32>,
    ) -> Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>> {
        self.sqs_client
            .receive_message()
            .set_queue_url(Some(self.queue_url.clone()))
            .set_max_number_of_messages(if max_messages.is_some() {
                max_messages
            } else {
                Some(1)
            })
            .send()
            .await
    }

    fn parse_json_message(&self, message: Option<&str>) -> Option<QueueMessage> {
        if let Some(json) = message {
            if let Ok(parsed) = serde_json::from_str::<QueueMessage>(json) {
                return Some(parsed);
            }
        }

        None
    }

    fn extract_queue_messages(
        &self,
        queue_output: Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>,
    ) -> Vec<QueueMessage> {
        let mut parsed_messages = Vec::new();

        if let Ok(message_output) = queue_output {
            println!("read_output: {:#?}", message_output);

            if let Some(unparsed_messages) = message_output.messages() {
                for unparsed_message in unparsed_messages {
                    if let Some(parsed_message) = self.parse_json_message(unparsed_message.body()) {
                        println!("parsed_message: {:#?}", parsed_message);

                        parsed_messages.push(parsed_message)
                    }
                }
            }
        }

        parsed_messages
    }

    pub async fn size(&self) -> u32 {
        if let Ok(attributes_output) = self
            .sqs_client
            .get_queue_attributes()
            .set_queue_url(Some(self.queue_url.clone()))
            .set_attribute_names(Some(vec![QueueAttributeName::ApproximateNumberOfMessages]))
            .send()
            .await
        {
            if let Some(count) = attributes_output
                .attributes()
                .unwrap()
                .get(&QueueAttributeName::ApproximateNumberOfMessages)
            {
                println!("count: {:#?}", count);

                return count.parse::<u32>().unwrap_or_default();
            }
        }

        0
    }

    pub async fn empty(&self) -> bool {
        self.size().await == 0
    }
}
