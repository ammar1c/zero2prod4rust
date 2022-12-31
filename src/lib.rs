
pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, post};
use std::net::TcpListener;
use serde::Deserialize;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(
        || {
            App::new()
                .route("/health_check", web::get().to(routes::health_check))
                .route("/subscriptions", web::post().to(routes::subscribe))
        }
    )
        .listen(listener)?
        .run();
    Ok(server)
}
