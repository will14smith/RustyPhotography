use rusoto_core::credential::{ProfileProvider, AwsCredentials};
use rusoto_dynamodb::DynamoDbClient;
use rusoto_core::{HttpClient, Region};
use photography::data;
use std::sync::Arc;
use photography::site_gen::{Site, image_provider};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::fs;
use rusoto_ssm::Ssm;
use std::collections::HashMap;

fn main() {
    let mut creds = ProfileProvider::new().unwrap();
    creds.set_profile("personal");

    let dynamo = DynamoDbClient::new_with(
        HttpClient::new().unwrap(),
        creds.clone(),
        Region::EuWest2
    );

    let ssm = rusoto_ssm::SsmClient::new_with(
        HttpClient::new().unwrap(),
        creds.clone(),
        Region::EuWest2
    );

    let params = ssm.get_parameters(rusoto_ssm::GetParametersRequest {
        names: vec!["photography-dev-SiteGeneratorSigningUser-AccessKey".into(), "photography-dev-SiteGeneratorSigningUser-SecretKey".into()],
        with_decryption: Some(true),
    }).sync().unwrap();

    let params: HashMap<String, String> = params.parameters.unwrap().iter().map(|p| (p.name.as_ref().unwrap().clone(), p.value.as_ref().unwrap().clone())).collect();

    let image_credentials = AwsCredentials::new(
        params.get("photography-dev-SiteGeneratorSigningUser-AccessKey").unwrap(),
        params.get("photography-dev-SiteGeneratorSigningUser-SecretKey").unwrap(),
        None, None
    );

    let client_config = data::Config {
        photograph_table: String::from("photography-dev-photograph"),
    };

    let image_provider_config = image_provider::S3ImageProviderConfig {
        bucket_name: "photography-dev-image".into(),
        region: Region::EuWest2,
        credentials: image_credentials,
    };

    let client = Arc::new(data::Client::new(dynamo, client_config));
    let image_provider = image_provider::S3ImageProvider::new(client.clone(), image_provider_config);
    let site_storer = LocalSiteStorer { output_folder: Path::new("temp").into() };

    photography::site_gen::generate(Arc::new(image_provider), Arc::new(site_storer), "static").unwrap();
}

struct LocalSiteStorer {
    output_folder: Box<Path>,
}

impl photography::site_gen::SiteStorer for LocalSiteStorer {
    fn store(&self, site: Site) -> Result<(), String> {
        for file in site.files() {
            let path = self.output_folder.join(file.name());
            fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

            let mut fs = File::create(path).map_err(|e| e.to_string())?;

            let content = file.generate();
            fs.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}