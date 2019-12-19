use photography::create_rocket;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_core::credential::ProfileProvider;
use rusoto_core::{HttpClient, Region};
use photography::data::Client;

fn main() {
    let mut creds = ProfileProvider::new().unwrap();
    creds.set_profile("personal");

    let dynamo = DynamoDbClient::new_with(
        HttpClient::new().unwrap(),
        creds,
        Region::EuWest2
    );

    let client = Client::new(dynamo);

    let r = create_rocket(client);

    r.launch();
}