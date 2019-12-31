use photography::create_rocket;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_core::credential::ProfileProvider;
use rusoto_core::{HttpClient, Region};
use photography::{ data, image_processing };
use std::sync::Arc;

fn main() {
    let mut creds = ProfileProvider::new().unwrap();
    creds.set_profile("personal");

    let dynamo = DynamoDbClient::new_with(
        HttpClient::new().unwrap(),
        creds.clone(),
        Region::EuWest2
    );

    let client_config = data::Config {
        photograph_table: String::from("photography-dev-photograph"),
    };

    let client = Arc::new(data::Client::new(dynamo, client_config));

    let sns = rusoto_sns::SnsClient::new_with(
        HttpClient::new().unwrap(),
        creds.clone(),
        Region::EuWest2
    );
    let notifier = Arc::new(image_processing::SnsNotifier::new(
        String::from("arn:aws:sns:eu-west-2:682179218046:photography-dev-imageprocessor"),
        Arc::new(sns)
    ));

    let r = create_rocket(client, notifier);

    r.launch();
}