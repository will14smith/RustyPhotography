use lambda_runtime::{
    Context,
    error::HandlerError,
    lambda,
};
use std::sync::Arc;
use serde::{ Deserialize, Serialize };
use photography::{config, site_gen::{
    self,
    image_provider
}, data};
use std::collections::HashMap;
use rusoto_ssm::Ssm;
use photography::site_gen::site_storer;

fn main() {
    lambda!(handler);
}

#[derive(Deserialize)]
struct Event {}
#[derive(Serialize)]
struct Output {}

fn handler(_: Event, _: Context) -> Result<Output, HandlerError> {
    let config = config::Config::from_env().expect("failed to get config");

    let image_provider = get_image_provider(&config);
    let site_storer = get_site_storer(&config);

    site_gen::generate(Arc::new(image_provider), Arc::new(site_storer), "/opt").map_err(|e| {
        HandlerError::from(e.as_str())
    })?;

    Ok(Output {})
}

fn get_data_client(config: &config::Config) -> data::Client {
    let dynamo = rusoto_dynamodb::DynamoDbClient::new(config.region());
    let client_config = data::Config {
        photograph_table: config.photograph_table().clone()
    };

    data::Client::new(dynamo, client_config)
}

fn get_image_credentials(config: &config::Config) -> rusoto_core::credential::AwsCredentials {
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

fn get_image_provider(config: &config::Config) -> image_provider::S3ImageProvider {
    let client = get_data_client(&config);

    let image_provider_config = image_provider::S3ImageProviderConfig {
        bucket_name: config.image_bucket().clone(),
        region: config.region(),
        credentials: get_image_credentials(&config),
    };

    image_provider::S3ImageProvider::new(Arc::new(client), image_provider_config)
}

fn get_site_storer(config: &config::Config) -> site_storer::S3SiteStorer {
    let s3 = rusoto_s3::S3Client::new(config.region());

    site_storer::S3SiteStorer::new(config.site_bucket().clone(), Arc::new(s3))
}