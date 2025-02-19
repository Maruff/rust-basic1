use serde::Deserialize;
use envy::FromEnv;

#[derive(Deserialize, FromEnv, Debug)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

pub fn load_config() -> Result<Config, envy::Error> {
    envy::from_env()
}
