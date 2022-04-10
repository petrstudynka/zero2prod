use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

use crate::routes::*;

pub fn run(listener: TcpListener, conn_pool: PgPool) -> Result<Server, std::io::Error> {
    let data_conn = web::Data::new(conn_pool);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data_conn.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
