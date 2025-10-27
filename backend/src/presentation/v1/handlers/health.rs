use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use anyhow::Context;

use crate::{infrastructure::applicaiton::ApplicationState, presentation::error::error_chain_fmt};

#[derive(thiserror::Error)]
pub enum HealthcheckError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error)
}

impl std::fmt::Debug for HealthcheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for HealthcheckError {
    fn status_code(&self) -> StatusCode {
        match self {
            HealthcheckError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// Basic health check endpoint
/// GET /api/v1/health
#[tracing::instrument(name = "Checking application health", skip_all)]
pub async fn health_check(
    app_state: web::Data<ApplicationState>
) -> Result<HttpResponse, HealthcheckError> {
    // Check PostgreSQL
    app_state
        .db_pool
        .health_check()
        .await
        .context("Postgres is unavailable")?;

    // Check Redis
    app_state
        .redis_client
        .health_check()
        .context("Redis is unavailable")?;

    Ok(HttpResponse::Ok().finish())
}
