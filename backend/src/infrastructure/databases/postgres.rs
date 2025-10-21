use std::error::Error;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::infrastructure::configuration::DatabaseSettings;

pub struct PostgresPool(Pool<Postgres>);

impl PostgresPool {
    #[allow(dead_code)]
    fn new(config: DatabaseSettings) -> Result<Self, Box<dyn Error>> {
        let pool = PgPoolOptions::new().connect_lazy_with(config.connect_options());
        Ok(Self(pool))
    }
}

impl AsRef<Pool<Postgres>> for PostgresPool {
    fn as_ref(&self) -> &Pool<Postgres> {
        &self.0
    }
}
