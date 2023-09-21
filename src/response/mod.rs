use actix_web::{web, HttpResponse};
use futures::{future::ok, stream::once};

async fn resp_stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::))
}
