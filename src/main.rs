use std::time::Duration;
use actix_web::{get, post, web, App, HttpResponse, Responder, HttpServer, Result};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/hello")]
async fn hello() -> impl Responder {
    std::thread::sleep(Duration::from_secs(5));
    "hello"
}

async fn scope() -> impl Responder {
    "hehe"
}

#[get("/user/{user_id}/info/{name}")]
async fn path_extract(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, name) = path.into_inner();
    Ok(format!("Welcom: {}, user_id: {}.", name, user_id))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/scope")
                    .route("/hehe", web::get().to(scope)),
            )
            .service(hello)
            .service(path_extract)
    })
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
