use serde::Serialize;
use crate::*;

#[derive(Serialize)]
struct MyResponse {
    name: String,
}

#[get("/resp/{name}")]
async fn get_response(name: web::Path<String>) -> Result<impl Responder> {
    let obj = MyResponse {
        name: name.to_string(),
    };
    Ok(web::Json(obj))
}

#[get("/respcompress")]
async fn get_resp_compress() -> HttpResponse {
    HttpResponse::Ok().body("data")
}