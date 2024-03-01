/*
    -- External tests --

    Tests in the external tests folder and doc tests, instead, have exactly the same level of access
    to your code that you would get if you were to add your crate as a dependency in another project.
    They are therefore used mostly for integration testing, i.e. testing your code by
    calling it in the same exact way a user would.
*/

// It is entirely decoupled from the underlying implementation details (except spawn_app)
#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8000/health-check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    // Launch the server as a background task
    // spawn takes a future and hands it over to the runtime for polling, without waiting
    // for its completion; it therefore runs concurrently with downstream futures and tasks
    let server = email_newsletter::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
