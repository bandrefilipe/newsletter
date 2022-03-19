use reqwest::StatusCode;
use std::net::TcpListener;

const LOCALHOST: &str = "127.0.0.1";

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
