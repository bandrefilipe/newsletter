use actix_web::{web, HttpResponse, Responder};

pub async fn subscribe(form: web::Form<SubscriptionForm>) -> impl Responder {
    dbg!(&form.name, &form.email);
    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
pub struct SubscriptionForm {
    name: String,
    email: String,
}
