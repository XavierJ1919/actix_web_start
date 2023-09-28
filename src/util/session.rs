use crate::*;
use actix_session::{Session};
use actix_web::Error;

pub async fn get_session(session: Session) -> Result<HttpResponse, Error> {
    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    Ok(HttpResponse::Ok().body(format!(
        "Count is {:?}!",
        session.get::<i32>("counter")?.unwrap()
    )))
}