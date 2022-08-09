use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

#[get("/healthcheck")]
async fn healthcheck() -> std::io::Result<impl Responder> {
    Ok(web::Json(MessageResponse {
        message: "healthy".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(healthcheck))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
