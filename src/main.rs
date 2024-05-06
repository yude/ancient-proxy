use actix_web::middleware::Logger;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use reqwest::{self, Client};

#[get("/{tail:.*}")]
async fn prx(
    request: HttpRequest,
    // path: web::Path<String>,
    // data: web::Data<AppState>,
) -> impl Responder {
    let client = Client::new();
    let res = client.get(request.uri().to_string()).send().await;

    match res {
        Ok(res) => {
            let body = res.text()
                .await
                .unwrap_or_else(|_| String::from("Failed to read response body"));
            HttpResponse::Ok().body(format!("{}", body))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
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
