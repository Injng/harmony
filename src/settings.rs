use std::path::Path;

use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub library: LibraryConfig,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct LibraryConfig {
    pub path: String,
}

impl Settings {
    pub fn load(path: &str) -> Self {
        let settings = Config::builder()
            .add_source(config::File::from(Path::new(path)))
            .add_source(config::Environment::with_prefix("HARMONY"))
            .build()
            .expect("[FATAL] Failed to read configuration file");
        settings
            .try_deserialize()
            .expect("[FATAL] Failed to deserialize configuration")
    }
}
