use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::infrastructure::applicaiton::ApplicationState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status:   String,
    pub postgres: HealthStatus,
    pub redis:    HealthStatus
}

#[derive(Serialize)]
pub struct HealthStatus {
    pub status:  String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>
}

/// Basic health check endpoint
/// GET /api/v1/health
pub async fn health_check(app_state: web::Data<ApplicationState>) -> impl Responder {
    // Check PostgreSQL
    let postgres_status = match app_state.db_pool.health_check().await {
        Ok(_) => HealthStatus {
            status:  "healthy".to_string(),
            message: None
        },
        Err(e) => HealthStatus {
            status:  "unhealthy".to_string(),
            message: Some(e.to_string())
        }
    };

    // Check Redis
    let redis_status = match app_state.redis_client.health_check() {
        Ok(_) => HealthStatus {
            status:  "healthy".to_string(),
            message: None
        },
        Err(e) => HealthStatus {
            status:  "unhealthy".to_string(),
            message: Some(e.to_string())
        }
    };

    // Overall status
    let overall_status = if postgres_status.status == "healthy" && redis_status.status == "healthy"
    {
        "healthy"
    } else {
        "degraded"
    };

    let response = HealthResponse {
        status:   overall_status.to_string(),
        postgres: postgres_status,
        redis:    redis_status
    };

    if overall_status == "healthy" {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}
