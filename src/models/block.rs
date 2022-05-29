use futures::stream::TryStreamExt;
use mongodb::{
    bson::doc,
    options::{FindOptions, IndexOptions},
    results as mongodb_results, Client, Collection, IndexModel,
};
use serde::{Deserialize, Serialize};

const DATABASE_NAME: &'static str = "naivecoin";

#[derive(Clone, Debug, Deserialize, Serialize)]
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

            match genesis_block.insert(&client).await {
                Err(error) => return Err(error),
                Ok(_) => (),
            };
            println!("successfully inserted genesis block");
        }

        Ok(())
    }

    pub async fn get_all(client: &Client) -> Result<Vec<Block>, &'static str> {
        let mut cursor = match Block::collection(&client).find(None, None).await {
            Err(err) => {
                println!("error getting all blocks: {:?}", err);
                return Err("failed to get all blocks");
            }
            Ok(value) => value,
        };

        let mut blocks: Vec<Block> = Vec::new();
        while let Some(block) = cursor.try_next().await.unwrap_or(None) {
            blocks.push(block);
        }

        return Ok(blocks);
    }
}

impl Block {
    pub async fn insert(
        &self,
        client: &Client,
    ) -> Result<mongodb_results::InsertOneResult, &'static str> {
        match Block::collection(&client).insert_one(self, None).await {
            Err(_) => Err("failed to insert block"),
            Ok(value) => Ok(value),
        }
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
