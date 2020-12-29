use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct Subscription {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<Subscription>) -> impl Responder {
    HttpResponse::Ok().body(format!("username: {}", form.name))
}
