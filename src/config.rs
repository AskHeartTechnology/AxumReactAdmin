use std::path::PathBuf;

use config::{Case, Config, Environment, File};
use dotenv;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub log: LogConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expire_hours: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dotenv::from_filename(base.join(".env")).expect("failed to load .env");

        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        let settings = Config::builder()
            .add_source(File::from(base.join("config/default")))
            .add_source(File::from(base.join(format!("config/{}", env))).required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__")
                    .try_parsing(true)
                    // ✅ 关键：让 APP_SERVER__PORT → server.port
                    .convert_case(Case::Lower),
            )
            .build()?;

        let config: Self = settings.try_deserialize()?;
        Ok(config)
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}
