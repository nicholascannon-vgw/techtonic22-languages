use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;
use serde::Serialize;
use std::{collections::HashMap, io::Result};

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

#[derive(Deserialize)]
struct WordCountBody {
    text: String,
}

#[get("/healthcheck")]
async fn healthcheck() -> Result<impl Responder> {
    Ok(web::Json(MessageResponse {
        message: "healthy".to_string(),
    }))
}

#[post("/count")]
async fn count_words(body: web::Json<WordCountBody>) -> Result<impl Responder> {
    let mut word_counts: HashMap<String, i64> = HashMap::new();

    // TODO: implementation

    Ok(web::Json(word_counts))
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // allow the frontend to call this
            .service(healthcheck)
            .service(count_words)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
