use std::collections::{BTreeSet, HashMap};

use mongodb::bson::DateTime;

use crate::db::{
    clips::Clip,
    twitch_messages::{get_vod_message_scores, VodMessageScore},
    twitch_vods::TwitchVodModel,
};

pub async fn analyze_vod(
    db_client: &mongodb::Database,
    vod: &TwitchVodModel,
) -> Result<Vec<Clip>, mongodb::error::Error> {
    let scores =
        get_vod_message_scores(db_client, &vod.channel_name, vod.streamed_at, vod.ended_at).await?;

    if scores.is_empty() {
        return Ok(Vec::new());
    }

    let bucket = create_bucket(scores);

    let average = calculate_weighted_bucket_average(&bucket);

    let outliers = find_outlier_timestamps(&bucket, average);

    Ok(create_clips(outliers, vod))
}

const TIME_BUCKET_SIZE_MS: u16 = 10_000;

/// TODO: switch to sliding window?
fn create_bucket(message_scores: impl IntoIterator<Item = VodMessageScore>) -> HashMap<i64, f64> {
    let mut bucket = HashMap::new();

    for score in message_scores {
        let timestamp_ms = score.timestamp.timestamp_millis();
        let key = timestamp_ms - (timestamp_ms % i64::from(TIME_BUCKET_SIZE_MS));

        bucket
            .entry(key)
            .and_modify(|v| *v += score.total_message_score)
            .or_insert(score.total_message_score);
    }

    bucket
}

fn find_outlier_timestamps(original_bucket: &HashMap<i64, f64>, average: f64) -> BTreeSet<i64> {
    let mut values = Vec::new();

    for s in original_bucket.values() {
        values.push(*s);
    }

    values.sort_by(|a, b| a.total_cmp(b));

    let top_5_percentage = values[(values.len() * 95) / 100] as i64;

    let base = TIME_BUCKET_SIZE_MS / 1000;

    let min = (std::cmp::max(
        top_5_percentage,
        std::cmp::max(average as i64, i64::from(base)),
    ) * 2) as f64;

    let mut outliers = BTreeSet::new();

    for (k, v) in original_bucket {
        if v > &min {
            outliers.insert(*k);
        }
    }

    outliers
}

/// Psuedo
fn calculate_weighted_bucket_average(bucket: &HashMap<i64, f64>) -> f64 {
    let mut total = 0.;
    let mut count = 0;

    // 1 message per second
    let min = f64::from(TIME_BUCKET_SIZE_MS) / 1000.;

    for s in bucket.values() {
        // ignore any bucket that has less than 1 message per second, since it makes the numbers unreliable
        if s >= &min {
            total += s;
            count += 1;
        }
    }

    total / f64::from(count)
}

#[inline]
fn timestamp_to_video_timestamp(timestamp: i64, start_time: &DateTime) -> i64 {
    timestamp - start_time.timestamp_millis()
}

fn create_clips(mut timestamps: BTreeSet<i64>, vod: &TwitchVodModel) -> Vec<Clip> {
    let mut clips = Vec::new();

    while !timestamps.is_empty() {
        if let Some(start_time) = timestamps.pop_first() {
            let mut end_time = start_time;

            while timestamps.contains(&(end_time + i64::from(TIME_BUCKET_SIZE_MS))) {
                end_time += i64::from(TIME_BUCKET_SIZE_MS);

                timestamps.remove(&end_time);
            }

            timestamps.remove(&start_time);

            clips.push(Clip {
                start_time: timestamp_to_video_timestamp(start_time, &vod.streamed_at),
                end_time: timestamp_to_video_timestamp(end_time, &vod.streamed_at),
                user_id: vod.user_id.clone(),
                vod_id: vod.vod_id.clone(),
                state: String::from("pending"),
            });
        }
    }

    clips
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::db::twitch_messages::VodMessageScore;

    use super::{calculate_weighted_bucket_average, create_bucket};

    #[test]
    fn test_create_bucket() {
        assert_eq!(
            HashMap::from([(0, 1.0), (5000, 2.0), (10000, 1.0)]),
            create_bucket([
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
            HashMap::from([(5000, 3.2), (10000, 2.1)]),
            create_bucket([
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
    fn test_calculate_bucket_average_score() {
        assert_eq!(
            4.0 / 3.0,
            calculate_weighted_bucket_average(&HashMap::from([
                (0, 1.0),
                (5000, 2.0),
                (10000, 1.0)
            ]))
        );

        assert_eq!(
            (123.12 + 21.121 + 512.12) / 3.0,
            calculate_weighted_bucket_average(&HashMap::from([
                (0, 123.12),
                (5000, 21.121),
                (10000, 512.12)
            ]))
        );
    }

    #[test]
    fn create_clips() {}
}
