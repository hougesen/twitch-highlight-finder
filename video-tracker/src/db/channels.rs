struct Channels {
    #[allow(dead_code)]
    channel_name: String,
    #[allow(dead_code)]
    channel_id: Option<String>,
}

pub async fn fetch_channels(
    db_client: &mongodb::Database,
) -> Result<Vec<String>, mongodb::error::Error> {
    let collection = db_client.collection::<Channels>("channels");

    let channel_id_queue = collection
        .distinct("channel_id", None, None)
        .await
        .unwrap_or_default()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    Ok(channel_id_queue)
}
