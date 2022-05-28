use super::models::block::Block;

use mongodb::Client;

pub struct Database {}

impl Database {
    pub async fn connect() -> Result<Client, &'static str> {
        let uri = std::env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

        let client = match Client::with_uri_str(uri).await {
            Err(_) => return Err("failed to connect"),
            Ok(value) => value,
        };

        match Block::create_index(&client).await {
            Err(error) => return Err(error),
            Ok(_) => (),
        };

        Ok(client)
    }
}
