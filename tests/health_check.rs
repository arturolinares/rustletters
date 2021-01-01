use uuid::Uuid;
use sqlx::Executor;
use sqlx::Connection;
use sqlx::PgConnection;
use rustletters::configuration::DatabaseSettings;
use sqlx::PgPool;

use rustletters::configuration::get_configuration;
use rustletters::startup::run;
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health-check", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name from subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=ObiWank%20Kenobi", "Missing the email"),
        ("email=obiwan%40jedi.net", "Missing the name"),
        ("", "Missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", app.address))
            .header("Content-Type", "appication/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

pub struct TestApp  {
    pub address: String,
    pub db_pool: PgPool
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let mut configuration = get_configuration("configuration").expect("Failed to read config file");
    // prepare a new test database
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = prepare_db(&configuration.database).await;
    // run the server
    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://localhost:{}", port),
        db_pool: connection_pool,
    }
}

async fn prepare_db(config: &DatabaseSettings) -> PgPool {
    let _connection = PgConnection::connect(&config.connection_string_nodb())
        .await
        .expect("Failed to connect to test database.")
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create test database.");

    let pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to test database.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the database");

    pool
}