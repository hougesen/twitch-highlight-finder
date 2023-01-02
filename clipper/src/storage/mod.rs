use aws_sdk_s3::{
    error::PutObjectError,
    output::PutObjectOutput,
    types::{ByteStream, SdkError},
};

pub const S3_BUCKET_NAME: &str = "twitch-highlight-finder-clips";

pub async fn setup_s3() -> aws_sdk_s3::Client {
    aws_sdk_s3::Client::new(&aws_config::load_from_env().await)
}

pub async fn upload_video(
    s3_client: &aws_sdk_s3::Client,
    file_name: &str,
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let stream = ByteStream::from_path(std::path::Path::new(&format!("clips/{file_name}.mp4")))
        .await
        .unwrap();

    s3_client
        .put_object()
        .bucket(S3_BUCKET_NAME)
        .key(format!("{file_name}.mp4"))
        .body(stream)
        .send()
        .await
}
