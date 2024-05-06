use actix_web::{HttpResponse};

pub async fn handle(res: reqwest::Response) -> actix_web::HttpResponse {
    let mut body = &res.text()
        .await
        .unwrap_or_else(|_| String::from("Failed to read response body"));

    HttpResponse::Ok().body(format!("{}", body))
}
