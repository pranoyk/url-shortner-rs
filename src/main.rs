mod urlstore;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub static URLSTORE: Lazy<Mutex<urlstore::UrlStore>> =
    Lazy::new(|| Mutex::new(urlstore::UrlStore::new()));

async fn shorten(url: web::Json<urlstore::ShortenRequest>) -> impl Responder {
    let short_url = URLSTORE.lock().unwrap().shorten(&url.url);
    HttpResponse::Ok().body(short_url)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(
            web::scope("/api").service(web::resource("/shorten").route(web::post().to(shorten))),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
