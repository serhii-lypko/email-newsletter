use std::net::TcpListener;

/*
    -- External tests --

    Tests in the external tests folder and doc tests, instead, have exactly the same level of access
    to your code that you would get if you were to add your crate as a dependency in another project.
    They are therefore used mostly for integration testing, i.e. testing your code by
    calling it in the same exact way a user would.
*/

// It is entirely decoupled from the underlying implementation details (except spawn_app)
// tokio::test spins up a new runtime at the beginning of each test case and they shut down at the end of each test case
// (no need to implement any clean up logic to avoid leaking resources between test runs.)
#[tokio::test]
async fn health_check_works() {
    // Spawned server will continue to run as long as the Tokio runtime is active
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // Launch the server as a background task
    // spawn takes a future and hands it over to the runtime for polling, without waiting
    // for its completion; it therefore runs concurrently with downstream futures and tasks

    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = email_newsletter::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
