use std::sync::{Mutex, MutexGuard};
use std::{
    collections::HashMap,
};
use lazy_static::lazy_static;
use log::{info};
use crate::game::components::user::user_component::{
    UserSocket, UserData,
};
use crate::database::redis::socket::socket_hash::{
    add_connecting_uuid_to_redis,
    remove_connecting_uuid_to_redis,
};

lazy_static! {
    static ref USER_SOCKETS: Mutex<HashMap<String, UserSocket>> = Mutex::new(HashMap::new());
}

pub fn add_user_socket(uuid: String, user_socket: UserSocket) {
    // redis에 접속 중인 uuid 추가
    let id = user_socket.id.clone();
    let name = user_socket.name.clone();
    USER_SOCKETS.lock().unwrap().insert(uuid.clone(), user_socket);
    tokio::spawn(async move {
        add_connecting_uuid_to_redis(UserData{
            uuid: uuid.clone(),
            id: id,
            name: name,
        }).await;
    });
}

pub fn delete_user_socket(uuid: String) {
    USER_SOCKETS.lock().unwrap().remove(&uuid.clone());
    tokio::spawn(async move {
        remove_connecting_uuid_to_redis(&uuid).await;
    });
}

pub fn get_user_socket() -> MutexGuard<'static, HashMap<String, UserSocket>> {
    USER_SOCKETS.lock().unwrap()
}