use rocket_lamb::RocketExt;
use photography::{ create_rocket, config, data };
use std::sync::Arc;

fn main() {
    let config = config::Config::from_env().expect("failed to get config");

    let client = get_data_client(&config);

    create_rocket(Arc::new(client))
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
