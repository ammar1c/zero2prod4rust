use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, post};
use std::net::TcpListener;
use serde::Deserialize;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
