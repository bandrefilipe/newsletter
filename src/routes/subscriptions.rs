use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;

pub async fn subscribe(
    form: web::Form<SubscriptionForm>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(e) => {
            eprintln!("Failed to insert a new subscription: {e}");
            HttpResponse::InternalServerError()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct SubscriptionForm {
    name: String,
    email: String,
}
