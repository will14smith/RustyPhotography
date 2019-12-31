use std::sync::Arc;

pub fn get_credentials() -> rusoto_core::credential::ProfileProvider {
    let mut creds = rusoto_core::credential::ProfileProvider::new().unwrap();
    creds.set_profile("personal");
    creds
}

pub fn get_dynamodb() -> Arc<dyn rusoto_dynamodb::DynamoDb + Send + Sync> {
    Arc::new(rusoto_dynamodb::DynamoDbClient::new_with(
        rusoto_core::HttpClient::new().unwrap(),
        get_credentials(),
        rusoto_core::Region::EuWest2
    ))
}

pub fn get_sns() -> Arc<dyn rusoto_sns::Sns + Send + Sync> {
    Arc::new(rusoto_sns::SnsClient::new_with(
        rusoto_core::HttpClient::new().unwrap(),
        get_credentials(),
        rusoto_core::Region::EuWest2
    ))
}

pub fn get_ssm() -> Arc<dyn rusoto_ssm::Ssm + Send + Sync> {
    Arc::new(rusoto_ssm::SsmClient::new_with(
        rusoto_core::HttpClient::new().unwrap(),
        get_credentials(),
        rusoto_core::Region::EuWest2
    ))
}

pub fn get_data_client() -> Arc<photography_data::Client> {
    Arc::new(photography_data::Client::new(
        get_dynamodb(),
        photography_data::Config { photograph_table: String::from("photography-dev-photograph") },
    ))
}

pub fn get_processor_notifier() -> Arc<dyn photography_processing::Notifier> {
    Arc::new(photography_processing::SnsNotifier::new(
        get_sns(),
        String::from("arn:aws:sns:eu-west-2:682179218046:photography-dev-imageprocessor"),
    ))
}

pub fn get_site_gen_image_provider(image_credentials: rusoto_core::credential::AwsCredentials) -> Arc<dyn photography_site_gen::ImageProvider> {
    Arc::new(photography_site_gen::s3_image_provider::S3ImageProvider::new(
        get_data_client(),
        photography_site_gen::s3_image_provider::S3ImageProviderConfig {
            bucket_name: String::from("photography-dev-image"),
            region: rusoto_core::Region::EuWest2,
            credentials: image_credentials,
        }
    ))
}