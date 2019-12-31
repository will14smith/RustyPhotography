use std::{ collections::HashMap, sync::Arc };
use rusoto_ssm::Ssm;

mod config;

pub use config::Config;

pub fn get_storage(config: &Config) -> Arc<dyn photography_processing::ImageStorage> {
    let s3 = Arc::new(rusoto_s3::S3Client::new(config.region()));

    Arc::new(photography_processing::S3ImageStorage::new(s3, config.image_bucket().clone()))
}

pub fn get_data_client(config: &Config) -> Arc<photography_data::Client> {
    let dynamo = Arc::new(rusoto_dynamodb::DynamoDbClient::new(config.region()));
    let client_config = photography_data::Config {
        photograph_table: config.photograph_table().clone()
    };

    Arc::new(photography_data::Client::new(dynamo, client_config))
}

fn get_image_credentials(config: &Config) -> rusoto_core::credential::AwsCredentials {
    let ssm = rusoto_ssm::SsmClient::new(config.region());

    let param_access_key = config.site_gen_param_access_key();
    let param_secret_key = config.site_gen_param_secret_key();

    let params = ssm.get_parameters(rusoto_ssm::GetParametersRequest {
        names: vec![param_access_key.clone(), param_secret_key.clone()],
        with_decryption: Some(true),
    }).sync().unwrap();

    let params: HashMap<String, String> = params.parameters.unwrap().iter()
        .map(|p| (p.name.as_ref().unwrap().clone(), p.value.as_ref().unwrap().clone())).collect();

    rusoto_core::credential::AwsCredentials::new(
        params.get(param_access_key).unwrap(),
        params.get(param_secret_key).unwrap(),
        None, None
    )
}

pub fn get_image_provider(config: &Config) -> Arc<dyn photography_site_gen::ImageProvider> {
    let client = get_data_client(&config);

    let image_provider_config = photography_site_gen::s3_image_provider::S3ImageProviderConfig {
        bucket_name: config.image_bucket().clone(),
        region: config.region(),
        credentials: get_image_credentials(&config),
    };

    Arc::new(photography_site_gen::s3_image_provider::S3ImageProvider::new(
        client,
        image_provider_config
    ))
}

pub fn get_site_storer(config: &Config) -> Arc<dyn photography_site_gen::SiteStorer> {
    let s3 = rusoto_s3::S3Client::new(config.region());

    Arc::new(photography_site_gen::s3_site_storer::S3SiteStorer::new(
        Arc::new(s3),
        config.site_bucket().clone()
    ))
}

pub fn get_notifier(config: &config::Config) -> Arc<dyn photography_processing::Notifier> {
    let sns = Arc::new(rusoto_sns::SnsClient::new(config.region()));

    Arc::new(photography_processing::SnsNotifier::new(
        sns,
        config.image_processor_topic().clone()
    ))
}
