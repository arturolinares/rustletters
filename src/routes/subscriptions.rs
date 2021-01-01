use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;


use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Subscription {
    name: String,
    email: String,
}

pub async fn subscribe(
	form: web::Form<Subscription>,
	pool: web::Data<PgPool>
) -> Result<HttpResponse, HttpResponse> {
	log::info!("Adding new subscriber {} ({}).", form.name, form.email);
	sqlx::query!(
		r#"
		INSERT INTO subscriptions (id, email, name, subscribed_at)
		VALUES ($1, $2, $3, $4);
		"#,
		Uuid::new_v4(),
		form.email,
		form.name,
		Utc::now()
	)
	.execute(pool.get_ref())
	.await
	.map_err(|e| {
		log::error!("Failed to create a new entry: {:?}", e);
		HttpResponse::InternalServerError().finish()
	})?;

	log::info!("New subscriber was saved successfully");

	Ok(HttpResponse::Ok().finish())
}
