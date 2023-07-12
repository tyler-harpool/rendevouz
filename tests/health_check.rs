use rendevouz::configuration::get_configuration;
use rendevouz::startup::run;
use rstest::rstest;
use std::net::SocketAddr;
use std::net::TcpListener;
use sqlx::PgPool;

#[rstest]
#[case("1.2.3.4:8080", 8080)]
#[case("127.0.0.1:9000", 9000)]
fn check_port(#[case] addr: SocketAddr, #[case] expected: u16) {
    assert_eq!(expected, addr.port());
}
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
// Launch our application in the background ~somehow~
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    )
    .await
    .expect("Failed to connect to Postgres");
    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }

   
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act
    let body = "name=Tyler%20Harpool&email=tylerharpool%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "tylerharpool@gmail.com");
    assert_eq!(saved.name, "Tyler Harpool")
}

#[tokio::test]
async fn subscribe_returns_a_400_for_valid_form_data() {
    //arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act
    let test_cases = vec![
        ("name=Tyler%20Harpool", "missing the name"),
        ("email=tylerharpool@gmail.com", "missing the email"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad request when the payload was {}",
            error_message,
        );
    }
}
