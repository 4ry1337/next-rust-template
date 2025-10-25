use actix_web::web;

pub mod handlers;
pub mod requests;
pub mod responses;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1").route("/health", web::get().to(handlers::health::health_check)) // Add more v1 routes here
                                                                                          // .service(
                                                                                          //     web::scope("/users")
                                                                                          //         .route("", web::post().to(user_handlers::create_user))
                                                                                          //         .route("", web::get().to(user_handlers::list_users))
                                                                                          //         .route("/{id}", web::get().to(user_handlers::get_user))
                                                                                          // )
    );
}
