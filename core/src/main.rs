mod server;

use actix_web::main as server_entry;

#[server_entry]
async fn main() -> std::io::Result<()> {
    server::listen().await
}
