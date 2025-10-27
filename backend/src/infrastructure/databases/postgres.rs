use sqlx::{postgres::PgPoolOptions, Error as SqlxError, Pool, Postgres};

use crate::infrastructure::configuration::DatabaseSettings;

pub struct PostgresPool(Pool<Postgres>);

impl PostgresPool {
    /// Creates a new PostgreSQL connection pool (lazy connection)
    pub fn new(config: &DatabaseSettings) -> Result<Self, anyhow::Error> {
        let pool = PgPoolOptions::new().connect_lazy_with(config.connect_options());
        Ok(Self(pool))
    }

    /// Health check - verifies database connection is working
    /// This executes a simple query to ensure the database is responsive
    #[tracing::instrument(name = "Checking database health", skip_all)]
    pub async fn health_check(&self) -> Result<(), SqlxError> {
        sqlx::query("SELECT 1").execute(&self.0).await?;
        Ok(())
    }

    /// Get the inner pool reference
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.0
    }
}

impl AsRef<Pool<Postgres>> for PostgresPool {
    fn as_ref(&self) -> &Pool<Postgres> {
        &self.0
    }
}
