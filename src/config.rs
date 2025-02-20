use serde::Deserialize;
use envy::from_env;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

pub fn load_config() -> Result<Config, envy::Error> {
    dotenv().ok(); // Load environment variables from .env file
    from_env::<Config>()
}
