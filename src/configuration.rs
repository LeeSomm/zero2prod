use secrecy::{ExposeSecret, SecretString};

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn connection_string_without_db(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize configuration builder
    let settings = config::Config::builder()
        // Search for file named `configuration`
        .add_source(config::File::with_name("configuration"))
        .build()?
        // Try to convert the configuration values into Settings type
        .try_deserialize()?;

    Ok(settings)
}
