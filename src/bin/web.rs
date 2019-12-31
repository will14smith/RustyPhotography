use rocket_lamb::RocketExt;
use photography::{ create_rocket, config, data, image_processing };
use std::sync::Arc;

fn main() {
    let config = config::Config::from_env().expect("failed to get config");

    let client = get_data_client(&config);
    let notifier = get_notifier(&config).into();

    create_rocket(Arc::new(client), notifier)
        .lambda()
        .launch();
}

fn get_data_client(config: &config::Config) -> data::Client {
    let dynamo = rusoto_dynamodb::DynamoDbClient::new(config.region());
    let client_config = data::Config {
        photograph_table: config.photograph_table().clone()
    };

    data::Client::new(dynamo, client_config)
}

fn get_notifier(config: &config::Config) -> Box<dyn image_processing::Notifier> {
    let topic_arn = config.image_processor_topic().clone();

    let sns = Arc::new(rusoto_sns::SnsClient::new(config.region()));

    Box::new(image_processing::SnsNotifier::new(topic_arn, sns))
}
