use std::io::Cursor;
use image::io::Reader as ImageReader;
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
