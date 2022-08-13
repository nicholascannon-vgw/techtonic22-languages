use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Serialize;
use std::env;
use std::{collections::HashMap, io::Result};

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

#[get("/healthcheck")]
async fn healthcheck() -> Result<impl Responder> {
    Ok(web::Json(MessageResponse {
        message: "healthy".to_string(),
    }))
}

#[post("/count")]
async fn count_words() -> Result<impl Responder> {
    let mut word_counts: HashMap<String, i64> = HashMap::new();
    word_counts.insert("Hello".to_string(), 1);
    word_counts.insert("world".to_string(), 1);

    Ok(web::Json(word_counts))
}

#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(healthcheck)
            .service(count_words)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
