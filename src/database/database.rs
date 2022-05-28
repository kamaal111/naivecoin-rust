use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

const DATABASE_NAME: &'static str = "naivecoin";

pub struct Database {}

impl Database {
    pub async fn connect() -> Result<Client, &'static str> {
        let uri = std::env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

        let client = match Client::with_uri_str(uri).await {
            Err(_) => return Err("failed to connect"),
            Ok(value) => value,
        };

        let _ = match User::create_index(&client).await {
            Err(error) => return Err(error),
            Ok(_) => (),
        };

        Ok(client)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}

impl User {
    fn collection_name() -> &'static str {
        "users"
    }

    async fn create_index(client: &Client) -> Result<(), &'static str> {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "username": 1 })
            .options(options)
            .build();

        let _ = match client
            .database(DATABASE_NAME)
            .collection::<User>(User::collection_name())
            .create_index(model, None)
            .await
        {
            Err(_) => return Err("failed to create user index"),
            Ok(_) => (),
        };

        Ok(())
    }
}
