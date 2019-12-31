use aws_lambda_events::event::sns;
use lambda_runtime::{
    Context,
    error::HandlerError,
    lambda,
};
use serde::Serialize;

fn main() {
    lambda!(handler);
}

#[derive(Serialize)]
struct Output {}

fn handler(event: sns::SnsEvent, _: Context) -> Result<Output, HandlerError> {
    let config = photography_lambda::Config::from_env().expect("failed to get config");

    let storage = photography_lambda::get_storage(&config);
    let data = photography_lambda::get_data_client(&config);

    let processor = photography_processing::Processor::new(storage, data);

    for record in event.records {
        process_record(&processor, &record.sns).map_err(|e| {
            HandlerError::from(e.as_str())
        })?;
    }

    Ok(Output { })
}

fn process_record(processor: &photography_processing::Processor, record: &sns::SnsEntity) -> Result<(), String> {
    let message = record.message.as_ref().ok_or("Missing message from record")?;
    let event: photography_processing::Event = serde_json::from_str(message.as_str()).map_err(|e| e.to_string())?;

    processor.process(event)?;

    Ok(())
}

