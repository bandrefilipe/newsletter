use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

pub async fn subscribe(
    form: web::Form<SubscriptionForm>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    let _ = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("request_id {request_id} - New subscriber details have been saved");
            HttpResponse::Ok()
        }
        Err(e) => {
            tracing::error!("request_id {request_id} - Failed to insert a new subscription: {e}");
            HttpResponse::InternalServerError()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct SubscriptionForm {
    name: String,
    email: String,
}
