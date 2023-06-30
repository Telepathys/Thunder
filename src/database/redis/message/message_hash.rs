use chrono::{FixedOffset, Utc};
use redis::Commands;
use crate::{database::redis::connect::connect_redis};

pub fn add_message_limit_list(uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.sadd("message_limit_list", &uid)?;
    let kst_offset = {
        let secs = 9 * 3600;
        FixedOffset::east_opt(secs).expect("FixedOffset::east out of bounds")
    };
    let now = Utc::now().with_timezone(&kst_offset);
    con.set(uid.to_owned()+"#message_limit_time", &now.to_rfc3339().to_string())?;
    con.smembers("message_limit_list")
}

pub fn exists_message_limit_list(uid: &String) -> redis::RedisResult<bool> {
    let mut con: redis::Connection = connect_redis()?;
    con.sismember("message_limit_list", &uid)
}

pub fn get_message_limit_time(uid: &String) -> redis::RedisResult<String> {
    let mut con: redis::Connection = connect_redis()?;
    con.get(uid.to_owned()+"#message_limit_time")
}

pub fn delete_message_limit_list(uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.srem("message_limit_list", &uid)?;
    con.del(uid.to_owned()+"#message_limit_time")?;
    con.smembers("message_limit_list")
}

pub fn add_message_history(uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    let kst_offset = {
        let secs = 9 * 3600;
        FixedOffset::east_opt(secs).expect("FixedOffset::east out of bounds")
    };
    let now = Utc::now().with_timezone(&kst_offset);
    con.sadd(uid.to_owned()+"#message_history", &now.to_rfc3339().to_string())?;
    con.smembers("message_history")
}

pub fn get_message_history(uid: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.smembers(uid.to_owned()+"#message_history")
}

pub fn get_message_history_count(uid: &String) -> redis::RedisResult<isize> {
    let mut con: redis::Connection = connect_redis()?;
    con.scard(uid.to_owned()+"#message_history")
}

pub fn delete_message_history(uid: &String,time: &String) -> redis::RedisResult<Vec<String>> {
    let mut con: redis::Connection = connect_redis()?;
    con.srem(uid.to_owned()+"#message_history", &time)?;
    con.smembers(uid.to_owned()+"#message_history")
}