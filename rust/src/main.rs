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

fn remove_from_string(string: String, character: &str) -> String {
    // We check contains first as the impl for `replace` is not very
    // efficient, it copies string data regardless if there's any change.
    // So we check if we would make a change to only change memory that needs
    // to be changed.
    if string.contains(character) {
        return string.replace(character, "");
    }

    return string;
}

#[post("/count")]
async fn count_words(body: web::Json<WordCountBody>) -> Result<impl Responder> {
    let words = body.text.split_whitespace();

    let word_counts = words.fold(HashMap::new(), |mut acc, curr| {
        let mut cleansed_word = curr.to_string();

        cleansed_word = remove_from_string(cleansed_word, "!");
        cleansed_word = remove_from_string(cleansed_word, ",");
        cleansed_word = remove_from_string(cleansed_word, "?");
        cleansed_word = remove_from_string(cleansed_word, "\n");
        cleansed_word = remove_from_string(cleansed_word, "\t");
        cleansed_word = remove_from_string(cleansed_word, "\\");
        cleansed_word = remove_from_string(cleansed_word, "#");

        if cleansed_word.trim() == "" {
            return acc;
        }

        // in-place manipulation
        *acc.entry(cleansed_word).or_insert(0) += 1;

        return acc;
    });

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
    .keep_alive(None)
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
