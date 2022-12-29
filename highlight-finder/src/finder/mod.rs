use crate::db::{
    twitch_messages::{get_vod_message_scores, VodMessageScore},
    twitch_vods::{mark_as_analyzed, TwitchVodModel},
};

#[derive(serde::Serialize)]
pub struct FinishedResult {
    median: f64,
    average: f64,
    outliers: Vec<i64>,
    bucket: std::collections::HashMap<i64, f64>,
}

pub async fn analyze_vod(
    db_client: &mongodb::Database,
    vod: TwitchVodModel,
) -> Result<(), mongodb::error::Error> {
    let scores =
        get_vod_message_scores(db_client, &vod.channel_name, vod.streamed_at, vod.ended_at).await?;

    println!("{} scores len: {:#?}", vod.id, scores.len());

    if scores.is_empty() {
        mark_as_analyzed(db_client, vod.id).await?;
        return Ok(());
    }

    let bucket = create_time_buckets(scores);

    let average = calculate_time_buckets_average_score(&bucket);

    if average < 10. {
        return Ok(());
    }

    let median = calculate_time_buckets_median_score(&bucket);

    let outliers = median_outliers(&bucket, average);

    std::fs::write(
        format!("./data/{}.json", vod.id),
        serde_json::to_string_pretty(&FinishedResult {
            median,
            average,
            outliers,
            bucket,
        })
        .unwrap(),
    )?;

    Ok(())
}

const TIME_BUCKET_SIZE: i64 = 15_000;

/// TODO: switch to sliding window?
pub fn create_time_buckets<T: IntoIterator<Item = VodMessageScore>>(
    message_scores: T,
) -> std::collections::HashMap<i64, f64> {
    let mut time_buckets = std::collections::HashMap::new();

    for score in message_scores {
        let timestamp_ms = score.timestamp.timestamp_millis();
        let key = timestamp_ms - (timestamp_ms % TIME_BUCKET_SIZE);

        time_buckets
            .entry(key)
            .and_modify(|v| *v += score.total_message_score)
            .or_insert(score.total_message_score);
    }

    time_buckets
}

pub fn calculate_time_buckets_median_score(
    time_bucket: &std::collections::HashMap<i64, f64>,
) -> f64 {
    let mut values = Vec::new();

    for s in time_bucket.values() {
        values.push(*s);
    }

    values.sort_by(|a, b| a.total_cmp(&b));

    values[values.len() / 2]
}

pub fn median_outliers(
    original_bucket: &std::collections::HashMap<i64, f64>,
    average: f64,
) -> Vec<i64> {
    let mut values = Vec::new();

    for s in original_bucket.values() {
        values.push(*s);
    }

    values.sort_by(|a, b| a.total_cmp(&b));

    let min = values[(values.len() / 10) * 9];

    let mut outliers = Vec::new();

    let average_with_multipler = average * 2.;

    for (k, v) in original_bucket {
        if v > &min && v > &average_with_multipler {
            outliers.push(*k);
        }
    }

    outliers
}

pub fn calculate_time_buckets_average_score(
    time_bucket: &std::collections::HashMap<i64, f64>,
) -> f64 {
    let mut total = 0.;

    for s in time_bucket.values() {
        total += s;
    }

    total / time_bucket.len() as f64
}

#[cfg(test)]
mod tests {
    use crate::db::twitch_messages::VodMessageScore;

    use super::{calculate_time_buckets_average_score, create_time_buckets};

    #[test]
    fn test_create_time_buckets() {
        assert_eq!(
            std::collections::HashMap::from([(0, 1.0), (5000, 2.0), (10000, 1.0)]),
            create_time_buckets([
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(0),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(5000),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(5001),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(10001),
                    count: 1,
                    total_message_score: 1.0,
                },
            ])
        );

        // test that order doesn't matter
        assert_eq!(
            std::collections::HashMap::from([(5000, 3.2), (10000, 2.1)]),
            create_time_buckets([
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(10001),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(10000),
                    count: 1,
                    total_message_score: 1.1,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(5000),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(5001),
                    count: 1,
                    total_message_score: 1.0,
                },
                VodMessageScore {
                    timestamp: mongodb::bson::DateTime::from_millis(5001),
                    count: 1,
                    total_message_score: 1.2,
                },
            ])
        );
    }

    #[test]
    fn test_calculate_time_buckets_average_score() {
        assert_eq!(
            4.0 / 3.0,
            calculate_time_buckets_average_score(&std::collections::HashMap::from([
                (0, 1.0),
                (5000, 2.0),
                (10000, 1.0)
            ]))
        );

        assert_eq!(
            (123.12 + 21.121 + 512.12) / 3.0,
            calculate_time_buckets_average_score(&std::collections::HashMap::from([
                (0, 123.12),
                (5000, 21.121),
                (10000, 512.12)
            ]))
        );
    }
}
