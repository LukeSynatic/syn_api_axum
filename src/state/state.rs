use std::env;

use mongodb::{options::ClientOptions, Client};

pub struct AppState {
    pub db: mongodb::Database
}

impl AppState {
    pub fn new(db: mongodb::Database) -> Self {
        AppState { db }
    }
}

#[derive(Debug, Clone)]
pub struct Mongo {
    pub db: mongodb::Database
}

impl Mongo {
    pub async fn new() -> Self {
        let mongo_uri = env::var("MONGODB_URI").expect("Error: Failed to get MONGO_URI from environment");
        let db_name = env::var("DB_NAME").expect("Error: Failed to get DB_NAME from environment");
        let client_options = ClientOptions::parse(mongo_uri).await.expect("Error: Failed to parse MongoDB client options");
        let client = Client::with_options(client_options).expect("Error: Failed to initialize MongoDB client with given options");
        let db = client.database(&db_name);

        Mongo { db }
    }
}