use std::net::TcpListener;

pub mod configuration;
mod domain;
pub mod routes;
pub mod startup;
pub mod telemetry;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::*;

pub fn run(listener: TcpListener, conn_pool: PgPool) -> Result<Server, std::io::Error> {
    let data_conn = web::Data::new(conn_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(data_conn.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
