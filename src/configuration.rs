use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
    pub tracing: TracingSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct TracingSettings {
    pub service_name: String,
    pub tracer_name: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        SecretString::new(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port,
                self.database_name
            )
            .into(),
        )
    }
}
