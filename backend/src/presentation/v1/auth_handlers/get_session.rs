use crate::infrastructure::middlewares::AuthenticatedUser;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};

use crate::{infrastructure::application::ApplicationState, presentation::error::error_chain_fmt};

#[derive(thiserror::Error)]
pub enum SessionError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error)
}

impl std::fmt::Debug for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SessionError {
    fn status_code(&self) -> StatusCode {
        match self {
            SessionError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

/// GET /api/v1/auth/me
#[tracing::instrument(name = "Getting current user data", skip_all)]
pub async fn get_session(
    _app_state: web::Data<ApplicationState>,
    user: web::ReqData<AuthenticatedUser>
) -> Result<HttpResponse, SessionError> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
            "id": user.id,
            "email": user.email,
            "name": user.name,
    })))
}
