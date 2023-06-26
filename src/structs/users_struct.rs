use serde::{Deserialize,Serialize};
use std::fmt;

#[derive(Deserialize)]
pub struct Join {
    pub id: String,
    pub pw: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Login {
    pub id: String,
    pub pw: String,
}

#[derive(Deserialize)]
pub struct TokenInput {
    pub uuid: String,
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct User {
    pub uuid: String,
    pub id: String,
    pub name: String,
    pub token: String,
}
