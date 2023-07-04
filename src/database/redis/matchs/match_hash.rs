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

pub fn add_match_join_list(match_id: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd("match_join_list", match_id)?;
    con.smembers("match_join_list")
}

pub fn add_match_wait_join_user_list(match_id: &String, uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd(match_id.to_owned()+"#wait", uid)?;
    con.smembers(match_id)
}

pub fn get_match_wait_join_user_list(match_id: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.smembers(match_id.to_owned()+"#wait")
}

pub fn add_match_join_user_list(match_id: &String, uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd(match_id.to_owned()+"#join", uid)?;
    con.smembers(match_id)
}

pub fn get_match_join_user_list(match_id: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.smembers(match_id.to_owned()+"#join")
}

pub fn check_match_exist(match_id: &String) -> redis::RedisResult<bool> {
    let mut con: redis::Connection = connect_redis()?;
    con.sismember("match_join_list", &match_id)
}