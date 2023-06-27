extern crate mongodb;
use mongodb::{bson::{doc, Document}};
use crate::structs::users_struct::{
    Join,
    Login,
};
use crate::utils::sha::{
    sha
};
use crate::database::mongo::connect::{
    connect_mongodb,
};

static COLL_NAME: &str = "users";

pub async fn user_join(join: Join) {
    let coll = connect_mongodb(COLL_NAME).await;

    let doc = doc! { 
        "uuid": uuid::Uuid::new_v4().to_string().trim_matches('"').to_string(),
        "id": join.id.trim_matches('"').to_string(),
        "pw": sha(&join.pw).trim_matches('"').to_string(),
        "name": join.name.trim_matches('"').to_string(),
    };

    coll.insert_one(doc, None).await.unwrap();
}

pub async fn find_by_name(name: &str) -> Document {
    let coll = connect_mongodb(COLL_NAME).await;
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
    let coll = connect_mongodb(COLL_NAME).await;
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