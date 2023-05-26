//! tests/health_check.rs
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works(){
    //Arrange
    let address = spawn_app();

    // use reqwest to perform HTTP reqwest to our Application
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check",&address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0),response.content_length());
}

//Launch the Application in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    
    //retrieving the random port
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    //Return the Applicaton address
    format!("http://127.0.0.1:{}",port)
}

