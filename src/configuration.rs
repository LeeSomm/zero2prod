#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize configuration reader
    let mut settings = config::Config::default();

    // Search for file named `configuration`
    // Will search for top-level files with compatible extension (yaml, json, etc.)
    settings.merge(config::File::with_name("configuration"))?;
}