use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::PgPool;

use crate::routes::{health_check, subscribe};

pub fn run_server(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    // wrap the connection pool in an ARC smart pointer
    let pool = web::Data::new(pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
