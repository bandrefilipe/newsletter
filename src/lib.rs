use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
struct SubscriptionForm {
    name: String,
    email: String,
}

async fn subscribe(form: web::Form<SubscriptionForm>) -> impl Responder {
    dbg!(&form.name, &form.email);
    HttpResponse::Ok()
}

pub fn run_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
