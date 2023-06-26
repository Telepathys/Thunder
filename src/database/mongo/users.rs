extern crate mongodb;
use mongodb::{Client, options::ClientOptions, bson::{doc, Document}};
use std::env;
use crate::structs::users_struct::{
    Join,
    Login,
};
use crate::utils::sha::{
    sha
};

static COLL_NAME: &str = "users";

async fn users() {
    let client_options = ClientOptions::parse(env::var("MONGO_URL").expect("Error: MONGO_URL not found")).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database(&env::var("MONGO_DB_NAME").expect("Error: MONGO_DB_NAME not found"));
    db.create_collection(COLL_NAME, None).await.unwrap();
}

pub async fn user_join(join: Join) {
    let client_options = ClientOptions::parse(env::var("MONGO_URL").expect("Error: MONGO_URL not found")).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&env::var("MONGO_DB_NAME").expect("Error: MONGO_DB_NAME not found"));
    let coll = db.collection(COLL_NAME);

    let doc = doc! { 
        "uuid": uuid::Uuid::new_v4().to_string().trim_matches('"').to_string(),
        "id": join.id.trim_matches('"').to_string(),
        "pw": sha(&join.pw).trim_matches('"').to_string(),
        "name": join.name.trim_matches('"').to_string(),
    };

    coll.insert_one(doc, None).await.unwrap();
}

pub async fn find_by_name(name: &str) -> Document {
    let client_options = ClientOptions::parse(env::var("MONGO_URL").expect("Error: MONGO_URL not found")).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&env::var("MONGO_DB_NAME").expect("Error: MONGO_DB_NAME not found"));
    let coll = db.collection::<Document>(COLL_NAME);
    let filter = doc! {"name": name};
    let result = coll.find_one(Some(filter), None).await.unwrap();
    match result {
        Some(doc) => return doc,
        None => {
            println!("No document found!");
            return doc! {}
        },
    }
}

pub async fn user_login(login: Login) -> Document {
    let client_options = ClientOptions::parse(env::var("MONGO_URL").expect("Error: MONGO_URL not found")).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&env::var("MONGO_DB_NAME").expect("Error: MONGO_DB_NAME not found"));
    let coll = db.collection::<Document>(COLL_NAME);
    let filter = doc! {
        "id": login.id,
        "pw": sha(&login.pw),
    };
    let result = coll.find_one(Some(filter), None).await.unwrap();
    match result {
        Some(doc) => return doc,
        None => {
            println!("No document found!");
            return doc! {}
        },
    }
}