// mod models;
// use models::blockchain::Blockchain;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

pub async fn listen() -> std::io::Result<()> {
    let port = 8080;

    println!("listening on {}", port);
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// fn main() {
//     let mut blockchain = Blockchain::new();
//     match blockchain.generate_next_block("data".to_string()) {
//         Err(error) => println!("error: {error:?}"),
//         Ok(_) => (),
//     }

//     println!("{:#?}", blockchain.blocks());
// }
