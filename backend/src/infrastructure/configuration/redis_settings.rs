use secrecy::{ExposeSecret, SecretString};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RedisSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port:          u16,
    pub host:          String,
    pub user:          String,
    pub password:      SecretString,
    pub database_name: String
}

impl RedisSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "redis://{}:{}@{}:{}/{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )
    }
}
