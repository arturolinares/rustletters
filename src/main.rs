use rustletters::startup::run;
use clap::App;
use clap::Arg;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = App::new("Rustletters")
        .author("Arturo Linares")
        .version("1.0")
        .about("Learnin application manage email subscriptions.")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .takes_value(true),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap_or("0");
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Error trying to bind server.");

    println!(
        "Starting server at http://localhost:{}",
        listener.local_addr().unwrap().port()
    );

    run(listener)?.await
}
