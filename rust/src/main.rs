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

    let words = body.text.split_whitespace();
    // word here references memory inside words
    for word in words {
        // Copy word out of words
        let mut cleansed_word = word.clone().to_string();

        if word.trim() == "" {
            continue;
        }
        if word.contains(",") {
            cleansed_word = cleansed_word.replace(",", "");
        }
        if word.contains("!") {
            cleansed_word = cleansed_word.replace("!", "")
        }

        let mut count = 0;

        let existing_count = word_counts.get(&cleansed_word);
        match existing_count {
            None => { /* Do nothing */ }
            Some(old_count) => count = *old_count,
        }

        count += 1;
        word_counts.insert(cleansed_word, count);
    }

    Ok(web::Json(word_counts))
}

#[actix_web::main]
async fn main() -> Result<()> {
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
