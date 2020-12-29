use crate::routes::subscriptions::subscribe;
use crate::routes::health_check::health_check;
use actix_web::HttpServer;
use actix_web::App;
use actix_web::web;
use actix_web::dev::Server;
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/health-check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();

    Ok(server)
}