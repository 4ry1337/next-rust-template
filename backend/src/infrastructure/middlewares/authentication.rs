use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error, HttpMessage, HttpResponse
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

/// User information extracted from JWT token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id:    String,
    pub email: String,
    pub name:  String
}

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub:   String, // Subject (user ID)
    email: String,
    name:  String,
    exp:   usize,         // Expiration time
    iat:   Option<usize>  // Issued at
}

/// JWKS (JSON Web Key Set) response structure
#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<Jwk>
}

#[derive(Debug, Deserialize)]
struct Jwk {
    kid: String,
    // kty:     String,
    // #[serde(rename = "use")]
    // key_use: String,
    // alg:     String,
    n:   String,
    e:   String
}

/// Authentication error types
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Missing authorization header")]
    MissingAuthHeader,

    #[error("Invalid authorization header format")]
    InvalidAuthHeader,

    #[error("Failed to fetch JWKS: {0}")]
    JwksFetchError(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String) // #[error("Missing user ID in token")]
                         // MissingUserId,
}

impl actix_web::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized",
            "message": self.to_string()
        }))
    }
}

/// Fetch JWKS from the authentication server
#[tracing::instrument(
    name = "Fetch JWKS",
    skip_all,
    fields(jwks_url = %jwks_url),
    err
)]
async fn fetch_jwks(jwks_url: &str) -> Result<JwksResponse, AuthError> {
    let client = reqwest::Client::new();

    let response = client
        .get(jwks_url)
        .send()
        .await
        .map_err(|e| AuthError::JwksFetchError(e.to_string()))?;

    let jwks = response
        .json::<JwksResponse>()
        .await
        .map_err(|e| AuthError::JwksFetchError(e.to_string()))?;

    tracing::debug!(keys_count = jwks.keys.len(), "JWKS fetched successfully");
    Ok(jwks)
}

/// Extract JWT token from Authorization header
#[tracing::instrument(name = "Extract token from header", skip_all, err)]
fn extract_token_from_header(req: &ServiceRequest) -> Result<String, AuthError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(AuthError::MissingAuthHeader)?
        .to_str()
        .map_err(|_| AuthError::InvalidAuthHeader)?;

    // Expected format: "Bearer <token>"
    let format = "Bearer ";

    if !auth_header.starts_with(format) {
        return Err(AuthError::InvalidAuthHeader);
    }

    tracing::debug!("Token extracted from Authorization header");
    Ok(auth_header[format.len()..].to_string())
}

/// Validate JWT token and extract user information
#[tracing::instrument(
    name = "Validate JWT token",
    skip_all,
    fields(jwks_url = %jwks_url),
    err
)]
async fn validate_token(token: &str, jwks_url: &str) -> Result<AuthenticatedUser, AuthError> {
    // Decode header to get the key ID (kid)
    let header = decode_header(token).map_err(|e| AuthError::InvalidToken(e.to_string()))?;

    tracing::debug!(kid = ?header.kid, "Decoded token header");

    // Fetch JWKS
    let jwks = fetch_jwks(jwks_url).await?;

    // Find the matching key
    let jwk = jwks
        .keys
        .iter()
        .find(|k| header.kid.as_ref() == Some(&k.kid))
        .ok_or_else(|| AuthError::InvalidToken("No matching key found".to_string()))?;

    tracing::debug!(kid = &jwk.kid, "Found matching JWK");

    // Convert JWK to DecodingKey (RSA public key)
    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
        .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

    // Validate token
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

    tracing::info!(
        user_id = %token_data.claims.sub,
        email = %token_data.claims.email,
        "Token validated successfully"
    );

    // Extract user information
    Ok(AuthenticatedUser {
        id:    token_data.claims.sub,
        email: token_data.claims.email,
        name:  token_data.claims.name
    })
}

/// Middleware function that rejects unauthenticated users
///
/// Usage:
/// ```rust
/// use actix_web::middleware::from_fn;
/// use crate::infrastructure::middlewares::authentication::reject_unauthenticated_users;
///
/// App::new()
///     .wrap(from_fn(reject_unauthenticated_users))
/// ```
#[tracing::instrument(
    name = "Authentication middleware",
    skip_all,
    fields(
        http.method = %req.method(),
        http.path = %req.path(),
    )
)]
pub async fn reject_unauthenticated_users(
    req: ServiceRequest,
    next: Next<impl MessageBody>
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let jwks_url = "http://localhost:3000/api/auth/jwks";

    let token = extract_token_from_header(&req)?;
    let user = validate_token(&token, jwks_url)
        .await
        .map_err(Error::from)?;

    tracing::info!(user_id = %user.id, "User authenticated successfully");

    req.extensions_mut().insert(user);
    next.call(req).await
}

/// Extension trait to easily get authenticated user from request
pub trait AuthenticatedUserExt {
    fn authenticated_user(&self) -> Option<AuthenticatedUser>;
}

impl AuthenticatedUserExt for actix_web::HttpRequest {
    fn authenticated_user(&self) -> Option<AuthenticatedUser> {
        self.extensions().get::<AuthenticatedUser>().cloned()
    }
}
