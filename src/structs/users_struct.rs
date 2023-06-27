use futures_channel::mpsc::UnboundedSender;
use serde::{Deserialize,Serialize};
use tokio_tungstenite::tungstenite::Message;
use std::{
    net::SocketAddr,
};

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

pub struct UserData {
    pub uuid: String,
    pub id: String,
    pub name: String,
}

pub struct UserSocket {
    pub id: String,
    pub name: String,
    pub socket: SocketAddr,
    pub tx: UnboundedSender<Message>,
}