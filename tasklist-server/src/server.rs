use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix_web_static_files;
use std::time::Instant;

use crate::session;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
        },
        &req,
        stream,
    )
}

pub async fn start() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .route("/ws", web::get().to(websocket))
            .service(
                actix_web_static_files::ResourceFiles::new("/", generated)
                    .resolve_not_found_to_root(),
            )
            .wrap(Logger::default())
    })
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
