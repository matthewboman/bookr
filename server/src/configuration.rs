use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username:      String, 
    pub password:      Secret<String>, 
    pub port:          u16,
    pub host:          String,
    pub database_name: String
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}", 
            self.username, 
            self.password.expose_secret(), 
            self.host, 
            self.port, 
            self.database_name
        ))
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database:         DatabaseSettings,
    pub application_port: u16
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("configuration.yml", config::FileFormat::Yaml))
        .build()?;

    settings.try_deserialize::<Settings>()
}