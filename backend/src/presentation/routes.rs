use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").configure(crate::presentation::v1::configure) // Add v2 routes here when ready
                                                                         // .configure(crate::presentation::v2::routes::configure)
    );
}
