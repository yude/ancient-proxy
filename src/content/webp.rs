use image::io::Reader as ImageReader;
use std::io::Cursor;
use actix_web::{HttpResponse};

pub async fn handle(res: reqwest::Response) -> actix_web::HttpResponse {
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
