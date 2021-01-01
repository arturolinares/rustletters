use sqlx::PgPool;
use crate::routes::health_check::health_check;
use crate::routes::subscribe;
use actix_web::dev::Server;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
	let db_pool = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
