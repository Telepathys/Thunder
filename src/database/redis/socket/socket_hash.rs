use log::info;
use redis::Commands;
use crate::{database::redis::connect::connect_redis, game::components::user::user_component::UserData};

pub async fn add_connecting_uuid_to_redis(user_data: UserData) -> redis::RedisResult<isize> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd("socket_list", &user_data.uuid)?;
    con.hset(&user_data.uuid, "uuid", &user_data.uuid)?;
    con.hset(&user_data.uuid, "id", &user_data.id)?;
    con.hset(&user_data.uuid, "name", &user_data.name)?;
    con.get("socket_list")
}

pub async fn remove_connecting_uuid_to_redis(uuid: &String) -> redis::RedisResult<isize> {
    let mut con: redis::Connection = connect_redis()?;
    info!("test: {}",uuid);
    con.srem("socket_list", uuid)?;
    con.del(uuid)?;
    con.get("socket_list")
}

pub async fn get_connecting_uuid_list() -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.smembers("socket_list")
}

pub async fn get_my_info(uuid: &String) -> redis::RedisResult<Vec<(String, String)>> {
    let mut con: redis::Connection = connect_redis()?;
    con.hgetall(uuid)
}