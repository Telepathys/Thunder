use redis::Commands;
use crate::{database::redis::connect::connect_redis};

pub fn check_group_exist(group_key: &String) -> redis::RedisResult<bool> {
    let mut con: redis::Connection = connect_redis()?;
    con.sismember("group_list", &group_key)
}

pub fn add_group_list(group_key: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd("group_list", group_key)?;
    con.smembers("group_list")
}

pub fn check_my_group(uid: &String) -> redis::RedisResult<bool> {
    let mut con: redis::Connection = connect_redis()?;
    con.exists(uid.to_owned()+"#group_key")
}

pub fn get_my_group_key(uid: &String) -> redis::RedisResult<String> {
    let mut con: redis::Connection = connect_redis()?;
    con.get(uid.to_owned()+"#group_key")
}

pub fn delete_group_list(group_key: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.srem("group_list", group_key)?;
    con.smembers("group_list")
}

pub fn this_group_join(group_key: &String, uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd(group_key, uid)?;
    con.set(uid.to_owned()+"#group_key", group_key)?;
    con.smembers(group_key)
}

pub fn this_group_leave(group_key: &String, uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.srem(group_key, uid)?;
    con.del(uid.to_owned()+"#group_key")?;
    con.smembers(group_key)
}

pub fn get_group_member(group_key: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.smembers(group_key)
}