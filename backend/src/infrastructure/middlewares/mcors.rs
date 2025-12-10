use actix_cors::Cors;

/// Creates a permissive CORS middleware.
///
/// Allows all origins, methods, and headers.
/// Useful for development.

pub fn cors() -> Cors {
    Cors::permissive()
}
