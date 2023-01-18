use queue::{self, Queue};

#[derive(serde::Deserialize)]
pub struct Event {
    pub kind: String,
    pub username: String,
}

pub async fn check_queue(queue: &Queue) -> Vec<Event> {
    let mut events = Vec::new();

    for message in queue
        .get_message_batch::<Event>(Some(10), Some(true))
        .await
        .unwrap_or_default()
    {
        events.push(message.0);
    }

    events
}
