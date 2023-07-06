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

pub fn delete_match_join_list(match_id: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.srem("match_join_list", match_id)?;
    con.smembers("match_join_list")
}

pub fn add_match_wait_join_user_list(match_id: &String, uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd(match_id.to_owned()+"#wait", uid)?;
    con.smembers(match_id.to_owned()+"#wait")
}

pub fn delete_match_wait_join_user_list(match_id: &String) -> redis::RedisResult<()> {
    let mut con: redis::Connection = connect_redis()?;
    con.del(match_id.to_owned()+"#wait")?;
    Ok(())
}

pub fn get_match_wait_join_user_list(match_id: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.smembers(match_id.to_owned()+"#wait")
}

pub fn add_match_join_user_list(match_id: &String, uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd(match_id.to_owned()+"#join", uid)?;
    con.smembers(match_id.to_owned()+"#join")
}

pub fn get_match_join_user_list(match_id: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.smembers(match_id.to_owned()+"#join")
}

pub fn delete_match_join_user_list(match_id: &String) -> redis::RedisResult<()> {
    let mut con: redis::Connection = connect_redis()?;
    con.del(match_id.to_owned()+"#join")?;
    Ok(())
}

pub fn check_match_exist(match_id: &String) -> redis::RedisResult<bool> {
    let mut con: redis::Connection = connect_redis()?;
    con.sismember("match_join_list", &match_id)
}

pub fn add_match(match_id: &String, uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd(match_id, uid)?;
    con.smembers(match_id)
}

pub fn get_match_members(match_id: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.smembers(match_id)
}

pub fn add_my_match(match_id: &String, uid: &String) -> redis::RedisResult<String> {
    let mut con: redis::Connection = connect_redis()?;
    con.set(uid.to_owned()+"#match_id", match_id)?;
    con.get(uid.to_owned()+"#match_id")
}

pub fn check_my_match_exist(uid: &String) -> redis::RedisResult<bool> {
    let mut con: redis::Connection = connect_redis()?;
    con.exists(uid.to_owned()+"#match_id")
}

pub fn add_match_list(match_id: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd("match_list", match_id)?;
    con.smembers("match_list")
}

pub fn get_my_match(uid: &String) -> redis::RedisResult<String> {
    let mut con: redis::Connection = connect_redis()?;
    con.get(uid.to_owned()+"#match_id")
}

pub fn add_match_response(match_id: &String, uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd(match_id.to_owned()+"#state", uid)?;
    con.smembers(match_id.to_owned()+"#state")
}

pub fn get_match_response(match_id: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.smembers(match_id.to_owned()+"#state")
}

pub fn delete_match_response(match_id: &String) -> redis::RedisResult<()> {
    let mut con: redis::Connection = connect_redis()?;
    con.del(match_id.to_owned()+"#state")?;
    Ok(())
}