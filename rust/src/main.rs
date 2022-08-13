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

    let words = body.text.split_ascii_whitespace();
    for word in words {
        // let mut cleansedWord = word;

        // if word.contains(",") {
        //     let new_s = cleansedWord.replace(",", "").to_string();
        //     cleansedWord = &new_s;
        // }
        // if word.contains("!") {
        //     cleansedWord = &cleansedWord.replace("!", "")
        // }
        // println!("{}", cleansedWord)

        let mut count = 0;

        let existing_count = word_counts.get(word);
        match existing_count {
            None => { /* Do nothing */ }
            Some(old_count) => count += old_count,
        }

        word_counts.insert(word.to_string(), count + 1);
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
