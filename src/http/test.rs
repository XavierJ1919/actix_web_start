// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, guard};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>,
}
struct AppState {
    app_name: String,
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world...")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there...")
}

async fn overslept() -> impl Responder {
    "overslept!"
}

async fn sleep2() -> impl Responder {
    "sleept2..."
}

#[get("/")]
async fn test_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("hello {app_name}")
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/scoped_config")
            .route(web::get().to(|| async { HttpResponse::Ok().body("re: scoped_config") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    println!("config");
    cfg.service(
        web::resource("/config")
            .route(web::get().to(|| async { HttpResponse::Ok().body("re: config") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            // .service(hello)
            .service(
                web::scope("/sleep")
                    .route("/overslept", web::get().to(overslept))
                    .guard(guard::Host("www.baidu.com"))
                    .route("/sleep2", web::get().to(sleep2)),
            )
            .route("/hey", web::get().to(manual_hello))
            .service(hello)
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix web..."),
            }))
            .app_data(counter.clone())
            .route("/index", web::get().to(index))
            .service(test_state)
            .configure(config)
            .service(web::scope("/scoped_config").configure(scoped_config))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
