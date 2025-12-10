use tracing_actix_web::{DefaultRootSpanBuilder, TracingLogger};

/// Creates the tracing logger middleware for request/response logging.
///
/// Automatically logs:
/// - HTTP method and path
/// - Response status code
/// - Request duration
/// - Integrates with the tracing spans configured in telemetry.rs
pub fn tracing_logger() -> TracingLogger<DefaultRootSpanBuilder> {
    TracingLogger::default()
}
