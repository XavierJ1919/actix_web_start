use crate::*;
use actix_web::{error, Error};
use serde::{Deserialize, Serialize};
use futures::StreamExt;
use serde_json::from_slice;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144;

#[post("/request/manual")]
async fn request_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        println!("Chunk: {:?}", &chunk);
        body.extend_from_slice(&chunk);
    }
    let obj = from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj))
}
