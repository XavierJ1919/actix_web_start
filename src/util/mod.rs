use actix_web::{get, guard, http::header, HttpRequest, HttpResponse, Responder, Result};

pub mod error;

#[get("/resource_url/")]
async fn resource_url(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("foo", ["1", "2", "3"])?;
    println!("url: {}", &url);

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url.as_str()))
        .finish())
}

// external resources
#[get("/external_resource")]
async fn external_resource(req: HttpRequest) -> impl Responder {
    let url = req.url_for("youtube", ["oHg5SJYRHA0"]).unwrap();
    assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");

    url.to_string()
}