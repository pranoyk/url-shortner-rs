mod urlstore;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn shorten(urlstore: web::Json<urlstore::ShortenRequest>) -> impl Responder {
    HttpResponse::Ok().body(urlstore.url.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(
            web::scope("/api")
                .service(
                    web::resource("/shorten")
                        .route(web::post().to(shorten))
                ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
