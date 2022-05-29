use super::database::Database;
use super::error_responses;
use super::mime;
use super::models::blockchain::Blockchain;

use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::Client;
use serde_json::Value;

#[derive(Debug, Clone)]
struct AppState {
    database_client: Option<Client>,
}

pub async fn listen() -> std::io::Result<()> {
    let port = 8080;
    println!("listening on {}", port);

    let connection: Option<Client> = match Database::connect().await {
        Err(error) => {
            println!("error connecting to database: {}", error);
            None
        }
        Ok(value) => Some(value),
    };

    let app = move || {
        let app_data = AppState {
            database_client: connection.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_data))
            .service(hello)
            .service(get_blocks)
            .service(mine_blocks)
    };

    let server = HttpServer::new(app).bind(("127.0.0.1", port));
    server?.run().await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .append_header((http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .body(format!("{{\"hello\": \"{}\"}}", "Kamaal"))
}

#[get("/blocks")]
async fn get_blocks(data: web::Data<AppState>) -> impl Responder {
    let database_client = data.database_client.clone().unwrap();
    let blockchain = Blockchain::new(&database_client);

    let blocks = blockchain.blocks().await.expect("failed to get blocks");

    HttpResponse::Ok()
        .append_header((http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .json(blocks)
}

#[post("/blocks")]
async fn mine_blocks(data: web::Data<AppState>, request_body: String) -> impl Responder {
    if request_body.len() < 2 {
        println!("error: invalid payload");
        return error_responses::bad_request();
    }

    let request_body: Value = match serde_json::from_str(&request_body) {
        Err(_) => {
            println!("error: invalid payload");
            return error_responses::bad_request();
        }
        Ok(value) => value,
    };

    let payload = match request_body.get("data") {
        None => {
            println!("error: invalid payload");
            return error_responses::bad_request();
        }
        Some(value) => match value.as_str() {
            None => {
                println!("error: invalid payload");
                return error_responses::bad_request();
            }
            Some(value) => value,
        },
    }
    .to_string();

    let database_client = data.database_client.clone().unwrap();
    let mut blockchain = Blockchain::new(&database_client);

    match blockchain.generate_next_block(payload) {
        Err(err) => {
            println!("error: {}", err);
            return error_responses::bad_request();
        }
        Ok(()) => (),
    };

    HttpResponse::NoContent().body("")
}
