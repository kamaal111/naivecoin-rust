use super::database::Database;
use super::error_responses;
use super::mime;
use super::models::blockchain::Blockchain;

use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::Client;
use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    latest: Option<u8>,
}

#[get("/blocks")]
async fn get_blocks(data: web::Data<AppState>, info: web::Query<AuthRequest>) -> impl Responder {
    let database_client = data.database_client.clone().unwrap();
    let blockchain = Blockchain::new(&database_client);

    let get_latest_block: bool = match info.latest {
        None => false,
        Some(0) => false,
        Some(1) => true,
        Some(_) => return error_responses::bad_request(),
    };

    if get_latest_block {
        let block = blockchain
            .get_latest_block()
            .await
            .expect("failed to get latest block");
        return HttpResponse::Ok()
            .append_header((http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
            .json(vec![block]);
    }

    let blocks = blockchain.blocks().await.expect("failed to get blocks");
    HttpResponse::Ok()
        .append_header((http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .json(blocks)
}

#[post("/blocks")]
async fn add_block_to_chain(data: web::Data<AppState>, request_body: String) -> impl Responder {
    let database_client = data.database_client.clone().unwrap();
    let blockchain = Blockchain::new(&database_client);

    match blockchain.add_to_chain_from_response(&request_body).await {
        Err(err) => {
            println!("error: {}", err);
            return error_responses::bad_request();
        }
        Ok(()) => (),
    };

    HttpResponse::NoContent().body("")
}

#[post("/mine")]
async fn mine_blocks(data: web::Data<AppState>, request_body: String) -> impl Responder {
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
    let blockchain = Blockchain::new(&database_client);

    match blockchain.generate_next_block(payload).await {
        Err(err) => {
            println!("error: {}", err);
            return error_responses::bad_request();
        }
        Ok(()) => (),
    };

    HttpResponse::NoContent().body("")
}
