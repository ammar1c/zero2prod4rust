
use actix_web::{HttpResponse, web};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct SubscribeData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(form: web::Form<SubscribeData>) -> HttpResponse {
    println!("received form {:?}", form);
    HttpResponse::Ok().finish()
}
