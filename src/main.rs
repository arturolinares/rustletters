use rustletters::telemetry::get_subscriber;
use rustletters::telemetry::init_subscriber;
use clap::{App, Arg};
use rustletters::configuration::get_configuration;
use rustletters::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("rustletters".into(), "info".into());
    init_subscriber(subscriber);

    let matches = App::new("Rustletters")
        .author("Arturo Linares")
        .version("1.0")
        .about("Learnin application manage email subscriptions.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("file")
                .takes_value(true),
        )
        .get_matches();



    let config_file = matches.value_of("config").unwrap_or("configuration");
    let configuration = get_configuration(config_file).expect("Failed to read configuration.");
    let port = configuration.application_port;
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Couldn't stablish a db connection");

    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Error trying to bind server.");

    println!(
        "Starting server at http://localhost:{}",
        listener.local_addr().unwrap().port()
    );

    run(listener, connection_pool)?.await
}

