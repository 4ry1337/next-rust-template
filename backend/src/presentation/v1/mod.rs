use actix_web::{middleware::from_fn, web};

use crate::infrastructure::middlewares::reject_unauthenticated_users;

mod auth_handlers;
mod health;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/health", web::get().to(health::health_check))
            .service(
                web::scope("/auth")
                    .wrap(from_fn(reject_unauthenticated_users))
                    .route("/verify", web::get().to(auth_handlers::get_session))
            ) // Add more v1 routes here
              // .service(
              //     web::scope("/users")
              //         .route("", web::post().to(user_handlers::create_user))
              //         .route("", web::get().to(user_handlers::list_users))
              //         .route("/{id}", web::get().to(user_handlers::get_user))
              // )
    );
}
