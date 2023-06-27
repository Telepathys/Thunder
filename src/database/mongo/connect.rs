extern crate mongodb;
use mongodb::{Client, Collection, options::ClientOptions, bson::{Document}};
use std::env;

pub async fn connect_mongodb(collsction: &str) -> Collection<Document> {
    let client_options = ClientOptions::parse(env::var("MONGO_URL").expect("Error: MONGO_URL not found")).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&env::var("MONGO_DB_NAME").expect("Error: MONGO_DB_NAME not found"));
    db.collection::<Document>(collsction)
}