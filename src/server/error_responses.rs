use super::mime;

use actix_web::{http, HttpResponse};

pub fn bad_request() -> HttpResponse {
    HttpResponse::BadRequest()
        .append_header((http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .body("{\"details\": \"Bad Request\"}")
}
