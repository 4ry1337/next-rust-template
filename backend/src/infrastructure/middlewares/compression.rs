use actix_web::middleware::Compress;

/// Creates the compression middleware for HTTP responses.
///
/// Automatically compresses responses using:
/// - Gzip (most common)
/// - Deflate
/// - Brotli (best compression ratio)
/// - Zstd
///
/// The client's Accept-Encoding header determines which algorithm is used.
pub fn compression() -> Compress {
    Compress::default()
}
