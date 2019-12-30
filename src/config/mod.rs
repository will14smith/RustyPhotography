use std::env;

pub struct Config {
    region: rusoto_core::Region,

    photograph_table: String,
}

impl Config {
    pub fn from_env() -> Result<Config, env::VarError> {
        Ok(Config {
            region: rusoto_core::Region::default(),

            photograph_table: env::var("PHOTOGRAPH_TABLE")?,
        })
    }

    pub fn region(&self) -> rusoto_core::Region {
        self.region.clone()
    }

    pub fn photograph_table(&self) -> &String {
        &self.photograph_table
    }
}