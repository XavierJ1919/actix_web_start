use actix_web::{error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "Internal error")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::BAD_REQUEST,
        }
    }
}

pub fn do_thing_that_fails() -> Result<(), String> {
    // Simulate an operation that can fail
    let result = 1;

    if result == 1 { /* some condition that indicates failure */
        // Return an error with an error message
        Err("An error occurred".to_string())
    } else {
        // Return success
        Ok(())
    }
}