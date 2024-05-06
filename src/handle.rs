use actix_web::{HttpRequest, HttpResponse};
use reqwest::{self, Client};

pub async fn handle(req: HttpRequest) -> HttpResponse {
    let client = Client::new();
    let res = client.get(req.uri().to_string()).send().await;

    match res {
        Ok(res) => {
            if let Some(content_type) = res.headers().get("Content-Type") {
                if let Ok(content_type_str) = content_type.to_str() {
                    match content_type_str {
                        "text/html" => {
                            crate::content::html::handle(res).await
                        },
                        "text/css" => {
                            HttpResponse::NotFound().body("This proxy does not accept text/css.")
                        }
                        "image/webp" => {
                            crate::content::webp::handle(res).await
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
