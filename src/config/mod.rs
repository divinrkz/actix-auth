use color_eyre::Result;
use eyre::WrapErr;
use serde::Deserialize;
use dotenv::dotenv;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub HOST: String,
    pub PORT: i32
}


impl Config {

    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();
        
        tracing_subscriber::fmt()
                        .with_env_filter(EnvFilter::from_default_env())
                        .init();

        info!("Loading Configurations");

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
            .context("Loading configuration from environment")
    }
}
