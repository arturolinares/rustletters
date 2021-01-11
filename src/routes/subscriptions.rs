use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct Subscription {
    name: String,
    email: String,
}

#[tracing::instrument(
	name = "Adding a new subscriber",
	skip(pool, form),
	fields(
		email = %form.email,
		name = %form.name
	)
)]
pub async fn subscribe(
    form: web::Form<Subscription>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    insert_subscriber(&pool, &form)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Insert subscriber into database", skip(pool, form))]
pub async fn insert_subscriber(pool: &PgPool, form: &Subscription) -> Result<(), sqlx::Error> {
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
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create a new entry: {:?}", e);
        e
    })?;

    Ok(())
}
