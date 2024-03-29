use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use std::convert::{TryFrom, TryInto};

use crate::domain::UserEmail;
use crate::email_client::EmailClient;
use crate::gmaps_api_client::GoogleMapsAPIClient;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database:     DatabaseSettings,
    pub application:  ApplicationSettings,
    pub email_client: EmailClientSettings,
    pub gmaps_client: GoogleMapsAPIClient,
    pub redis_uri:    Secret<String>,
    pub jwt_settings: JWTSettings,
    pub frontend_url: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port:           u16,
    pub host:           String,
    pub base_url:       String,
    pub hmac_secret:    Secret<String>,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username:      String, 
    pub password:      Secret<String>, 
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port:          u16,
    pub host:          String,
    pub database_name: String,
    pub require_ssl:   bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailClientSettings {
    pub base_url:       String,
    pub sender_email:   String,
    pub auth_token:     Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timeout_millis: u64
}

impl EmailClientSettings {
    pub fn sender(&self) -> Result<UserEmail, String> {
        UserEmail::parse(self.sender_email.clone())
    }

    pub fn timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout_millis)
    }

    pub fn client(self) -> EmailClient {
        let sender_email = self.sender().expect("Invalid sender email address");
        let timeout      = self.timeout();

        EmailClient::new(
            self.base_url,
            sender_email,
            self.auth_token,
            timeout
        )
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct JWTSettings {
    pub secret:     Secret<String>,
    pub expires_in: String,
    pub max_age:    u64
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!("{} is not a supported environment. Use either `local` or `production`", other))
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path  = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let environment_filename     = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base.yaml")))
        .add_source(config::File::from(config_dir.join(environment_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__")
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}