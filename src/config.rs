use serde::Deserialize;
use std::sync::RwLock;

// https://stackoverflow.com/questions/61159698/update-re-initialize-a-var-defined-in-lazy-static
lazy_static! {
    static ref CONFIG: RwLock<Option<Config>> = RwLock::new(None);
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub auth_issuer: String,
    pub auth_client_id: String,
    pub auth_allowed_domains: Vec<String>,
}

pub fn get() -> Result<Config, String> {
    // Get from the cache first
    if let Some(cfg) = CONFIG.read().unwrap().to_owned() {
        return Ok(cfg);
    }

    match envy::prefixed("CODEHQ_TS_API_").from_env::<Config>() {
        Ok(cfg) => {
            let mut new_config = CONFIG.write().unwrap();
            *new_config = Some(cfg.clone());
            Ok(cfg)
        }
        Err(err) => Err(err.to_string()),
    }
}
