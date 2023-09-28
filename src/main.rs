use std::sync::Mutex;
use std::time::Duration;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{get, post, web, App, http::header, HttpRequest, HttpResponse, Responder, HttpServer, Result, guard, middleware, middleware::Logger, cookie::Key};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Deserialize;
use env_logger::Env;

use util::error::UserError;
use crate::util::error::do_thing_that_fails;
use crate::http::resource::{external_resource, resource_url};
use crate::http::request::request_manual;
use crate::http::response::{get_resp_compress, get_response};
use crate::util::session::get_session;

mod http;
mod util;

#[derive(Deserialize)]
struct DecodedData {
    username: String,
}

#[derive(Deserialize)]
struct BodyInfo {
    flavor: String,
    size: u16,
    name_list: Vec<String>,
    // num_array: [u8;3],
}

#[derive(Deserialize)]
struct QueryInfo {
    username: String,
}

struct Payload {
    tablename: String,
}

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

// async fn show_table()
async fn state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("hello {app_name}")
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    std::thread::sleep(Duration::from_secs(5));
    "hello"
}

async fn sleep() -> impl Responder {
    "sleep"
}

async fn overslept() -> impl Responder {
    "Re: overslept"
}

async fn scope() -> impl Responder {
    "hehe"
}

#[get("/user/{user_id}/info/{name}")]
async fn path_extract(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, name) = path.into_inner();
    Ok(format!("Welcom: {}, user_id: {}.", name, user_id))
}

#[get("/getinfo")]
async fn get_username(info: web::Query<QueryInfo>) -> String {
    let username = &info.username;
    format!("get query: {}", username)
}

#[post("/submit")]
async fn submit_info(info: web::Json<BodyInfo>) -> Result<String> {
    Ok(format!("get body from request, flavor: {}, size: {}, name_list: {:#?}", info.flavor, info.size, info.name_list))
    // Ok(format!("get body from request, flavor: {}, size: {}, num_array: {:#?}", info.flavor, info.size, info.num_array))
}

#[post("/decodeurl")]
async fn decode_url(info: web::Form<DecodedData>) -> Result<String> {
    Ok(format!("Re, get decoded data: {}", info.username))
}

#[get("/testerror")]
async fn test_error() -> Result<&'static str, UserError> {
    do_thing_that_fails().map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/scope")
                    .route("/hehe", web::get().to(scope)),
            )
            .service(
                web::scope("/sleep")
                    .route("/overslept", web::get().to(overslept))
                    .guard(guard::Host("www.baidu.com"))
                    .route("/sleep", web::get().to(sleep)),
            )
            .service(hello)
            .service(path_extract)
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix web xj..."),
            }))
            // 根据测试，app_data只在之后起作用，而guard与顺序无关；需要文档验证；
            .app_data(counter.clone())
            .route("/index", web::get().to(index))
            .route("/state", web::get().to(state))
            .service(get_username)
            .service(submit_info)
            .service(decode_url)
            .service(test_error)
            .service(
                web::resource("/resource_url/{a}/{b}/{c}")
                    .name("foo")
                    .guard(guard::Get())
                    .to(HttpResponse::Ok),
            )
            .service(resource_url)
            .service(external_resource)
            .external_resource("youtube", "https://youtube.com/watch/{videio_id}")
            .service(request_manual)
            .service(get_response)
            .wrap(middleware::Compress::default())
            .service(get_resp_compress)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                .cookie_secure(false)
                .build()
            )
            .service(web::resource("/get_session").to(get_session))
    })
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
