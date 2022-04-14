use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    name = "Posting new subscription",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(
    form: web::Form<SubscriptionForm>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match insert_subscription(&form, &pool).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[tracing::instrument(name = "Inserting new subscription in the database", skip(form, pool))]
async fn insert_subscription(form: &SubscriptionForm, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to insert new subscription: {:?}", err);
        err
    })?;
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct SubscriptionForm {
    name: String,
    email: String,
}
