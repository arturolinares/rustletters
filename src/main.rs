use rustletters::telemetry::get_subscriber;
use rustletters::telemetry::init_subscriber;
use sqlx::postgres::PgPoolOptions;

use rustletters::configuration::get_configuration;
use rustletters::startup::run;

use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("rustletters".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let port = configuration.application.port;
    let host = configuration.application.host;

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Couldn't stablish a db connection");

    let listener =
        TcpListener::bind(format!("{}:{}", host, port)).expect("Error trying to bind server.");

    println!(
        "Starting server at http://localhost:{}",
        listener.local_addr().unwrap().port()
    );

    run(listener, connection_pool)?.await
}
