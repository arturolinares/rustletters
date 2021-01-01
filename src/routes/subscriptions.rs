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
		println!("Failed to execute query: {}", e);
		HttpResponse::InternalServerError()
			.body(e.to_string())
	})?;

	Ok(HttpResponse::Ok().finish())
}
