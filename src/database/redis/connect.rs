extern crate redis;
use redis::Commands;
use std::env;

pub fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open(env::var("REDIS_URL").expect("Error: REDIS_URL not found"))?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _ : () = con.set("my_key1", 43)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}