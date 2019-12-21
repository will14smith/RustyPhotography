use photography::create_rocket;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_core::credential::ProfileProvider;
use rusoto_core::{HttpClient, Region};
use photography::data;

fn main() {
    let mut creds = ProfileProvider::new().unwrap();
    creds.set_profile("personal");

    let dynamo = DynamoDbClient::new_with(
        HttpClient::new().unwrap(),
        creds,
        Region::EuWest2
    );

    let client_config = data::Config {
        photograph_table: String::from("photography-dev-photograph"),
    };

    let client = data::Client::new(dynamo, client_config);

    let r = create_rocket(client);

    r.launch();
}