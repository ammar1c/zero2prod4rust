
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct SubscribeData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(form: web::Form<SubscribeData>,
        pool: web::Data<PgPool>) -> HttpResponse {
    println!("received form {:?}", form);
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        .execute( pool.get_ref())
        .await
        .expect("Failed to execute query.");
    HttpResponse::Ok().finish()
}
