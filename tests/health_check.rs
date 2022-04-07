use std::net::TcpListener;

use reqwest::StatusCode;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use newsletter::configuration::DatabaseConfig;
use newsletter::{configuration, startup};

const LOCALHOST: &str = "127.0.0.1";

struct TestApp {
    address: String,
    db_pool: PgPool,
}

/// Spins up an instance of our application
/// and returns a [TestApp] instance.
async fn spawn_app() -> TestApp {
    let listener =
        TcpListener::bind(format!("{LOCALHOST}:0")).expect("Failed to bind a random port.");
    let port = listener.local_addr().unwrap().port();

    let mut config = configuration::parse().expect("Failed to parse the application config");
    let connection_pool = configure_database(&mut config.database).await;

    let server =
        startup::run_server(listener, connection_pool.clone()).expect("Failed launch the server");
    // runs the server as a background task, allowing our tests to run concurrently
    tokio::spawn(server);

    TestApp {
        address: format!("http://{LOCALHOST}:{port}"),
        db_pool: connection_pool,
    }
}

/// Manipulates the [DatabaseConfig] to create and connect to a new random logical database,
/// so we can guarantee test isolation.
async fn configure_database(config: &mut DatabaseConfig) -> PgPool {
    // Create database
    config.dbname = Uuid::new_v4().to_string();
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to the database");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.dbname).as_str())
        .await
        .expect("Failed to create a new database");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to the database");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
async fn subscriptions_returns_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = "name=Andr%C3%A9%20Filipe&email=b.andrefilipe%40gmail.com";

    // Act
    let response = client
        .post(format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(request_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "b.andrefilipe@gmail.com");
    assert_eq!(saved.name, "André Filipe");
}

#[tokio::test]
async fn subscriptions_returns_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Andr%C3%A9%20Filipe", "missing the email"),
        ("email=b.andrefilipe%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, invalid_reason) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "POST /subscriptions dit not fail with 400 Bad Request when the payload was {}.",
            invalid_reason
        );
    }
}
