#[path = "models/mod.rs"]
mod models;
use models::blockchain::Blockchain;

use actix_web::{get, http, App, HttpResponse, HttpServer, Responder};

pub async fn listen() -> std::io::Result<()> {
    let port = 8080;
    println!("listening on {}", port);

    let app = || App::new().service(hello).service(get_blocks);
    HttpServer::new(app).bind(("127.0.0.1", port))?.run().await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .append_header((http::header::CONTENT_TYPE, "application/json"))
        .body("{\"hello\": \"world\"}")
}

#[get("/blocks")]
async fn get_blocks() -> impl Responder {
    let blockchain = Blockchain::new();
    let blocks = serde_json::to_string(&blockchain.blocks()).unwrap();

    HttpResponse::Ok()
        .append_header((http::header::CONTENT_TYPE, "application/json"))
        .body(blocks)
}

// fn main() {
//     let mut blockchain = Blockchain::new();
//     match blockchain.generate_next_block("data".to_string()) {
//         Err(error) => println!("error: {error:?}"),
//         Ok(_) => (),
//     }
// }
