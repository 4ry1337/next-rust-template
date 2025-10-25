use secrecy::{ExposeSecret, SecretString};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RedisSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port:     u16,
    pub host:     String,
    pub username: String,
    pub password: SecretString,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub db:       u16,
    pub uri:      SecretString
}

impl RedisSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "redis://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.db
        )
    }
}
