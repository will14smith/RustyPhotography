use std::{
    collections::HashMap,
    fs::{ self, File },
    io::Write,
    path::Path,
    sync::Arc,
};
use rusoto_core::credential::AwsCredentials;

fn main() {
    let ssm = photography_local::get_ssm();
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

    let image_provider = photography_local::get_site_gen_image_provider(image_credentials);
    let site_storer = Arc::new(LocalSiteStorer { output_folder: Path::new("temp").into() });

    photography_site_gen::generate(image_provider, site_storer, "static").unwrap();
}

struct LocalSiteStorer {
    output_folder: Box<Path>,
}

impl photography_site_gen::SiteStorer for LocalSiteStorer {
    fn store(&self, site: photography_site_gen::Site) -> Result<(), String> {
        for file in site.files {
            let path = self.output_folder.join(file.name());
            fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

            let mut fs = File::create(path).map_err(|e| e.to_string())?;

            let content = file.generate();
            fs.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}