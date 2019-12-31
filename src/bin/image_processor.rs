use serde::Serialize;
use lambda_runtime::{
    Context,
    error::HandlerError,
    lambda,
};
use aws_lambda_events::event::sns;
use photography::{ config, data, image_processing };
use std::sync::Arc;

fn main() {
    lambda!(handler);
}

#[derive(Serialize)]
struct Output {}

fn handler(event: sns::SnsEvent, _: Context) -> Result<Output, HandlerError> {
    let config = config::Config::from_env().expect("failed to get config");

    let storage = get_storage(&config);
    let data = get_data_client(&config);

    let processor = image_processing::Processor::new(storage, data);

    for record in event.records {
        process_record(&processor, &record.sns).map_err(|e| {
            HandlerError::from(e.as_str())
        })?;
    }

    Ok(Output { })
}

fn get_storage(config: &config::Config) -> Arc<dyn image_processing::ImageStorage> {
    let s3 = Arc::new(rusoto_s3::S3Client::new(config.region()));

    Arc::new(image_processing::S3ImageStorage::new(config.image_bucket().clone(), s3))
}

fn get_data_client(config: &config::Config) -> Arc<data::Client> {
    let dynamo = rusoto_dynamodb::DynamoDbClient::new(config.region());
    let client_config = data::Config {
        photograph_table: config.photograph_table().clone()
    };

    Arc::new(data::Client::new(dynamo, client_config))
}

fn process_record(processor: &image_processing::Processor, record: &sns::SnsEntity) -> Result<(), String> {
    let message = record.message.as_ref().ok_or("Missing message from record")?;
    let event: image_processing::Event = serde_json::from_str(message.as_str()).map_err(|e| e.to_string())?;

    processor.process(event)?;

    Ok(())
}

