use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, post};
use std::net::TcpListener;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct SubscribeData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<SubscribeData>) -> HttpResponse {
    println!("received form {:?}", form);
    HttpResponse::Ok().finish()
}

