use std::env;

pub struct Config {
    photograph_table: String,
}

impl Config {
    pub fn from_env() -> Result<Config, env::VarError> {
        Ok(Config {
            photograph_table: env::var("PHOTOGRAPH_TABLE")?,
        })
    }

    pub fn photograph_table(&self) -> &String {
        &self.photograph_table
    }
}