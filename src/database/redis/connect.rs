extern crate redis;
use log::{error};
use redis::{Commands, PubSubCommands, ControlFlow};
use tokio_tungstenite::tungstenite::Message;
use std::{env, thread};
use crate::{game::components::redis::redis_component::{
    RedisSubscribe,
    RedisPublish,
}, router::server::socket_router::socket_router};

pub fn connect_redis() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open(env::var("REDIS_URL").expect("Error: REDIS_URL not found"))?;
    // docker-compose
    // let client = redis::Client::open("redis://redis:6379")?;
    client.get_connection()
}

pub fn redis_subscribe() {
    let mut con = connect_redis().unwrap();
    con.subscribe(&["thunder"], |msg| {
        let payload: String = msg.get_payload().unwrap();
        match payload.as_ref() {
            "10" => ControlFlow::Break(()),
            _ => {
                tokio::task::spawn_blocking(move || {
                    match serde_json::from_str::<RedisSubscribe>(&payload) {
                        Ok(redis_subscribe) => {
                            if !redis_subscribe.msg.is_empty() {
                                socket_router(redis_subscribe.uid, tokio_tungstenite::tungstenite::Message::Text(redis_subscribe.msg));
                            }
                        }
                        Err(err) => {
                            error!("Deserialization error: {}", err);
                        }
                    }
                });
                ControlFlow::Continue
            }
        }
    }).unwrap();
}

pub async fn redis_publish(uid: String, msg: Message) {
    thread::spawn(move || {
        let mut con = connect_redis().unwrap();
        if !msg.is_empty() {
            let redis_publish = RedisPublish {
                uid: uid,
                msg: msg.to_string(),
            };
            let redis_publish_serialized = serde_json::to_string(&redis_publish).unwrap();
            let _: () = con.publish("thunder", redis_publish_serialized).unwrap();
        }
    });
}