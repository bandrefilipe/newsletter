use reqwest::StatusCode;
use std::net::TcpListener;

const LOCALHOST: &str = "127.0.0.1";

/// Spins up an instance of our application
/// and returns its address (i.e. http://localhost:0)
fn spawn_app() -> String {
    let listener =
        TcpListener::bind(format!("{LOCALHOST}:0")).expect("Failed to bind a random port.");
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run_server(listener).expect("Failed launch the server.");
    // runs the server as a background task, allowing our tests to run concurrently
    tokio::spawn(server);

    format!("http://{LOCALHOST}:{port}")
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let base_url = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{base_url}/health"))
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
    let base_url = spawn_app();
    let client = reqwest::Client::new();
    let request_body = "name=Andr%C3%A9%20Filipe&email=b.andrefilipe%40gmail.com";

    // Act
    let response = client
        .post(format!("{base_url}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(request_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn subscriptions_returns_400_when_data_is_missing() {
    // Arrange
    let base_url = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Andr%C3%A9%20Filipe", "missing the email"),
        ("email=b.andrefilipe%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, invalid_reason) in test_cases {
        // Act
        let response = client
            .post(format!("{base_url}/subscriptions"))
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
