use secrecy::{ExposeSecret, SecretString};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DatabaseSettings {
    pub username:      String,
    pub password:      SecretString,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port:          u16,
    pub host:          String,
    pub database_name: String,
    pub require_ssl:   bool
}

impl DatabaseSettings {
    pub fn connect_options(&self) -> PgConnectOptions {
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
            .database(&self.database_name)
    }
}
