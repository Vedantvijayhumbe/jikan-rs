use actix_web::{get, web, HttpResponse, Responder};
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

struct Anime {
    mal_id: u32,
    title: String,
    synopsis: Option<String>,
    url: String,
    images: Option<ImageData>,
}

struct ImageData {
    jpg: Option<ImageFormats>,
    webp: Option<ImageFormats>,
}

struct ImageFormats {
    image_url: String,
}
struct JikanRandomResponse {
    data: Anime,
}

#[get("/random/anime")]
async fn get_random_anime() -> impl Responder {
    let url = "https://api.jikan.moe/v4/random/anime";
    
    match fetch_random_anime(url).await {
        Ok(anime) => HttpResponse::Ok().json(anime),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

async fn fetch_random_anime(api_url: &str) -> Result<Anime, Box<dyn Error>> {
    let response = reqwest::get(api_url).await?;
    
    if response.status().is_success() {
        let json: JikanRandomResponse = response.json().await?;
        Ok(json.data)
    } else {
        Err("Failed to fetch anime from Jikan API".into())
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_random_anime);
}
