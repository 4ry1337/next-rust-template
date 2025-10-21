use std::error::Error;

use redis::Client;

use crate::infrastructure::configuration::RedisSettings;

pub struct RedisClient(Client);

impl RedisClient {
    #[allow(dead_code)]
    fn new(config: RedisSettings) -> Result<Self, Box<dyn Error>> {
        let client = Client::open(config.connection_string())?;
        Ok(Self(client))
    }
}

impl AsRef<Client> for RedisClient {
    fn as_ref(&self) -> &Client {
        &self.0
    }
}
