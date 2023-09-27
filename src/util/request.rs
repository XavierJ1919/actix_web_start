use crate::*;
use actix_web::{error, Error};
use serde::{Deserialize, Serialize};
use futures::StreamExt;
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144;

#[post("/")]
async fn request_manuel(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    // let obj = serde_json::
    Ok(HttpResponse::Ok().json(obj))
}
