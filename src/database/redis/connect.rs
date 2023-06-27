extern crate redis;
use redis::Commands;
use std::env;

pub fn fetch_an_integer() -> redis::RedisResult<isize> {
    let mut con = connect_redis()?;
    let _ : () = con.set("my_key1", 43)?;
    con.get("my_key")
}

pub fn connect_redis() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open(env::var("REDIS_URL").expect("Error: REDIS_URL not found"))?;
    client.get_connection()
}