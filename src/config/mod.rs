use std::env;

pub struct Config {
    region: rusoto_core::Region,

    photograph_table: String,
    image_bucket: String,
    site_bucket: String,

    site_gen_param_access_key: String,
    site_gen_param_secret_key: String,
}

impl Config {
    pub fn from_env() -> Result<Config, env::VarError> {
        Ok(Config {
            region: rusoto_core::Region::default(),

            photograph_table: env::var("PHOTOGRAPH_TABLE")?,
            image_bucket: env::var("IMAGE_BUCKET")?,
            site_bucket: env::var("SITE_BUCKET")?,

            site_gen_param_access_key: env::var("SSM_SiteGenerator_AccessKey")?,
            site_gen_param_secret_key: env::var("SSM_SiteGenerator_SecretKey")?,
        })
    }

    pub fn region(&self) -> rusoto_core::Region {
        self.region.clone()
    }

    pub fn photograph_table(&self) -> &String {
        &self.photograph_table
    }
    pub fn image_bucket(&self) -> &String {
        &self.image_bucket
    }
    pub fn site_bucket(&self) -> &String {
        &self.site_bucket
    }

    pub fn site_gen_param_access_key(&self) -> &String {
        &self.site_gen_param_access_key
    }
    pub fn site_gen_param_secret_key(&self) -> &String {
        &self.site_gen_param_secret_key
    }
}