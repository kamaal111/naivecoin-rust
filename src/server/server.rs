use super::models::blockchain::Blockchain;

#[path = "mime.rs"]
mod mime;

use actix_web::{get, http, post, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;

pub async fn listen() -> std::io::Result<()> {
    let port = 8080;
    println!("listening on {}", port);

    let app = || {
        App::new()
            .service(hello)
            .service(get_blocks)
            .service(mine_blocks)
    };
    HttpServer::new(app).bind(("127.0.0.1", port))?.run().await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .append_header((http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .body("{\"hello\": \"world\"}")
}

#[get("/blocks")]
async fn get_blocks() -> impl Responder {
    let blockchain = Blockchain::new();
    let blocks = serde_json::to_string(&blockchain.blocks()).unwrap();

    HttpResponse::Ok()
        .append_header((http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .body(blocks)
}

#[post("/blocks")]
async fn mine_blocks(request_body: String) -> impl Responder {
    if request_body.len() < 2 {
        println!("error: invalid payload");
        return bad_request();
    }

    let request_body: Value = match serde_json::from_str(&request_body) {
        Err(_) => {
            println!("error: invalid payload");
            return bad_request();
        }
        Ok(value) => value,
    };

    let data = match request_body.get("data") {
        None => {
            println!("error: invalid payload");
            return bad_request();
        }
        Some(value) => match value.as_str() {
            None => {
                println!("error: invalid payload");
                return bad_request();
            }
            Some(value) => value,
        },
    }
    .to_string();

    let mut blockchain = Blockchain::new();
    match blockchain.generate_next_block(data) {
        Err(err) => {
            println!("error: {}", err);
            return bad_request();
        }
        Ok(()) => (),
    };

    HttpResponse::NoContent().body("")
}

fn bad_request() -> HttpResponse {
    HttpResponse::BadRequest()
        .append_header((http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .body("{\"details\": \"Bad Request\"}")
}
