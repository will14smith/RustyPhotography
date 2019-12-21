use rocket_lamb::RocketExt;
use photography::{ create_rocket, config, data };
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

fn main() {
    let config = config::Config::from_env().expect("failed to get config");

    let dynamo = DynamoDbClient::new(Region::default());
    let client_config = data::Config {
        photograph_table: config.photograph_table().clone()
    };

    let client = data::Client::new(dynamo, client_config);

    create_rocket(client)
        .lambda()
        .launch();
}