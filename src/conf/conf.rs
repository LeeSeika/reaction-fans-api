use config::{Config as Conf, ConfigError};
use serde::Deserialize;
use super::{db::Database, oauth::QQ, server::Server};


#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: Server,

    pub database: Database,
    
    pub oauth_qq: QQ,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let settings = Conf::builder()
            // add ./config/theme-api.toml
            .add_source(config::File::with_name("./config.toml"))
            // merge with environment variables
            .add_source(config::Environment::default())
            .build()?;

        let conf = settings.try_deserialize::<Config>()?;
        
        Ok(conf)
    }
}