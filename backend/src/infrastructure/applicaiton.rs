use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Compress, web, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_actix_web::TracingLogger;

use crate::{
    infrastructure::{
        configuration::{DatabaseSettings, Settings},
        databases::{postgres::PostgresPool, redis::RedisClient}
    },
    presentation::configure
};

pub struct Application {
    port:   u16,
    server: Server
}

pub struct ApplicationState {
    pub db_pool:      PostgresPool,
    pub redis_client: RedisClient,
    pub settings:     Settings
}

impl Application {
    /// Build the application with health checks and retry logic
    pub async fn build(settings: Settings) -> Result<Self, anyhow::Error> {
        tracing::info!("Building application...");
        let db_pool = PostgresPool::new(&settings.database)?;
        let redis_client = RedisClient::new(&settings.redis)?;

        let app_state = web::Data::new(ApplicationState {
            settings: settings.clone(),
            db_pool,
            redis_client
        });

        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );

        tracing::info!("Binding server to {}", address);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr()?.port();

        let server = run(listener, app_state)?;

        tracing::info!("Application built successfully on port {}", port);

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        tracing::info!("Starting server on port {}", self.port);
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

pub fn run(
    listener: TcpListener,
    app_state: web::Data<ApplicationState>
) -> Result<Server, anyhow::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(TracingLogger::default())
            .wrap(Compress::default())
            .configure(configure)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
