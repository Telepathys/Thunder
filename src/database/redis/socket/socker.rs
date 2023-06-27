use redis::Commands;

use crate::database::redis::connect::connect_redis;

pub async fn add_socket() -> redis::RedisResult<isize> {
    let mut con = connect_redis()?;
    let _ : () = con.set("my_key1", 43)?;
    con.get("my_key")
}