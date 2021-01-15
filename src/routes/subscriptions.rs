use crate::domain::NewSubscriber;
use crate::domain::SubscriberName;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
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
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let new_subscriber = NewSubscriber {
        name: SubscriberName::parse(form.0.name)?,
        email: form.0.email,
    };
    insert_subscriber(&pool, &new_subscriber)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Insert subscriber into database", skip(pool, new_subscriber))]
pub async fn insert_subscriber(pool: &PgPool, new_subscriber: &NewSubscriber) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
		INSERT INTO subscriptions (id, email, name, subscribed_at)
		VALUES ($1, $2, $3, $4);
		"#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create a new entry: {:?}", e);
        e
    }).expect("Name validation failed");

    Ok(())
}
