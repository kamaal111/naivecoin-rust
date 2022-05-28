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

        let amount_of_blocks = match collection.count_documents(None, None).await {
            Err(_) => return Err("failed to create blocks index"),
            Ok(value) => value,
        };

        if amount_of_blocks == 0 {
            let genesis_block = Block {
                index: 0,
                hash: "cd2fb2ace926608315b2a5bd1bc2a259dce057a21ed63351adc0b1326da2a99e"
                    .to_string(),
                parent_hash: None,
                timestamp: 1652722519,
                data: "The Genesis block!!!".to_string(),
            };

            match collection.insert_one(genesis_block, None).await {
                Err(_) => return Err("failed to insert genesis block"),
                Ok(_) => (),
            };
            println!("successfully inserted genesis block");
        }

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
