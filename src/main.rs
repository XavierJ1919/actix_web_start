use std::time::Duration;
use actix_web::{get, post, web, App, HttpResponse, Responder, HttpServer};

#[get("/hello")]
async fn hello() -> impl Responder {
    std::thread::sleep(Duration::from_secs(5));
    "hello"
}

async fn scope() -> impl Responder {
    "hehe"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/scope")
                    .route("/hehe", web::get().to(scope)),
            )
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
