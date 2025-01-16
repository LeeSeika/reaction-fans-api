use super::{cache::Cache, db::Database, mailer::Mailer, nats::Nats, oauth::QQ, server::Server};
use config::{Config as Conf, ConfigError};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: Server,

    pub database: Database,

    pub cache: Cache,

    // pub oauth_qq: QQ,

    // pub nats: Nats,
    pub mailer: Mailer,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let settings = Conf::builder()
            // add ./config/theme-api.toml
            .add_source(config::File::with_name("./config/config.toml"))
            // merge with environment variables
            .add_source(config::Environment::default())
            .build()?;

        let conf = settings.try_deserialize::<Config>()?;

        Ok(conf)
    }
}
