use rocket_lamb::RocketExt;
use photography::create_rocket;
use photography::data::Client;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

fn main() {
    let dynamo = DynamoDbClient::new(Region::default());
    let client = Client::new(dynamo);

    create_rocket(client)
        .lambda()
        .launch();
}