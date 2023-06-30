use redis::Commands;
use crate::{database::redis::connect::connect_redis};

pub fn add_random_match_wait_list(uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd("random_match_wait_list", &uid)?;
    con.smembers("random_match_wait_list")
}

pub fn check_random_match_wait_list(uid: &String) -> redis::RedisResult<bool> {
    let mut con: redis::Connection = connect_redis()?;
    con.sismember("random_match_wait_list", &uid)
}

pub fn delete_random_match_wait_list(uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.srem("random_match_wait_list", &uid)?;
    con.smembers("random_match_wait_list")
}

pub fn get_random_match_wait_list() -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.smembers("random_match_wait_list")
}