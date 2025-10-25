use redis::{Client, Connection, RedisError};

use crate::infrastructure::configuration::RedisSettings;

pub struct RedisClient(Client);

impl RedisClient {
    /// Creates a new Redis client
    pub fn new(config: &RedisSettings) -> Result<Self, anyhow::Error> {
        println!("{}", config.connection_string());
        let client = Client::open(config.connection_string())?;
        Ok(Self(client))
    }

    /// Get a connection from the client
    pub fn get_connection(&self) -> Result<Connection, RedisError> {
        self.0.get_connection()
    }

    /// Get the inner client reference
    pub fn client(&self) -> &Client {
        &self.0
    }

    pub fn health_check(&self) -> Result<(), RedisError> {
        let mut conn = self.0.get_connection()?;
        redis::cmd("PING").query::<String>(&mut conn)?;
        Ok(())
    }
}

impl AsRef<Client> for RedisClient {
    fn as_ref(&self) -> &Client {
        &self.0
    }
}
