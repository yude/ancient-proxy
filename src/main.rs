use actix_web::middleware::Logger;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use reqwest::{self, Client};
use std::io::Cursor;
use image::io::Reader as ImageReader;

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
            if let Some(content_type) = res.headers().get("Content-Type") {
                if let Ok(content_type_str) = content_type.to_str() {
                    match content_type_str {
                        "text/html" => {
                            let body = &res.text()
                                .await
                                .unwrap_or_else(|_| String::from("Failed to read response body"));
                            HttpResponse::Ok().body(format!("{}", body))
                        },
                        "text/css" => {
                            HttpResponse::NotFound().body("This proxy does not accept text/css.")
                        }
                        "image/webp" => {
                            let b = &res.bytes()
                                .await
                                .unwrap();

                            let img = ImageReader::new(Cursor::new(b))
                                .with_guessed_format()
                                .expect("Failed to guess image format")
                                .decode()
                                .expect("Failed to decode image");
                            
                            let mut converted: Vec<u8> = Vec::new();
                            let _ = img.write_to(&mut Cursor::new(&mut converted), image::ImageFormat::Png);

                            HttpResponse::Ok().body(converted)
                        }
                        _ => {
                            let b = &res.bytes()
                                .await
                                .unwrap();
                            HttpResponse::Ok().body(b.to_vec())
                        }
                    }
                } else {
                    HttpResponse::InternalServerError().finish()
                }
            } else {
                HttpResponse::InternalServerError().finish()
            }
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
