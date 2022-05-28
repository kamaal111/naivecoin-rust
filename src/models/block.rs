use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use serde::Serialize;

const DATABASE_NAME: &'static str = "naivecoin";

#[derive(Clone, Serialize)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub parent_hash: Option<String>,
    pub timestamp: u64,
    pub data: String,
}

impl Block {
    pub async fn create_index(client: &Client) -> Result<(), &'static str> {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "index": 1 })
            .options(options)
            .build();

        let collection = Block::collection(&client);
        match collection.create_index(model, None).await {
            Err(_) => return Err("failed to create blocks index"),
            Ok(_) => (),
        };

        Ok(())
    }
}

impl Block {
    fn collection_name() -> &'static str {
        "blocks"
    }

    fn collection(client: &Client) -> Collection<Block> {
        client
            .database(DATABASE_NAME)
            .collection::<Block>(Block::collection_name())
    }
}
