use actix_web::middleware::Logger;
use actix_web::{get, App, HttpRequest, HttpServer, Responder};

mod handle;
mod content;

#[get("/{tail:.*}")]
async fn prx(
    req: HttpRequest,
) -> impl Responder {
    handle::handle(req).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(prx)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
