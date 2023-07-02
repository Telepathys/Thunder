use std::{
    env,
};
use crate::database::redis::connect::{redis_subscribe};
use tokio::net::{TcpListener};
use log::{info,error};
use tonic::{transport::Server};
use thunder::{hello_server};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use crate::router::server::http_router::{
    join,
    login,
    asd,
};
use crate::socket::{
    handle_connection,
};
use crate::grpc::thunder::test::hello_service::{
    HelloService, thunder,
};
use crate::game::scheduler::scheduler::scheduler_core;


pub async fn server_start() {
     // logger level setting
    env::set_var("RUST_LOG", "info");
    env_logger::init();
     // dotenv using
    dotenv().ok();

     // Socket server start
    let addr = env::args().nth(1).unwrap_or_else(|| "0.0.0.0:7777".to_string());
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    tokio::spawn(async move {
        info!("socket server start : {}", addr);
         // socket main loop
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(handle_connection(stream, addr));
        }
    });

     // HTTP server start
    let http_addr = env::args().nth(2).unwrap_or_else(|| "0.0.0.0:7778".to_string());
    let server = HttpServer::new(|| {
        App::new()
            .service(join)
            .service(login)
            .service(asd)
    })
    .bind(&http_addr).unwrap()
    .run();
    tokio::spawn(async move {
        info!("HTTP server start : {}", http_addr);
        if let Err(err) = server.await {
            error!("HTTP server error: {}", err);
        }
    });

     // gRPC server start
    let hello_service = HelloService::default();
    let addr = "0.0.0.0:7779".parse().unwrap();
    let hello_server = hello_server::HelloServer::new(hello_service);
    tokio::spawn(async move {
        info!("gRPC server start : {}", addr);
        if let Err(err) = Server::builder()
            .add_service(hello_server)
            .serve(addr)
            .await
        {
            error!("gRPC server error : {}", err);
        }
    });

     // redis channel subscribe
    tokio::spawn(async move {
        info!("redis channel subscribe start");
        redis_subscribe();
    });

     // scheduler system
    tokio::spawn(async move {
        info!("scheduler system start");
        scheduler_core().await
    });


    tokio::signal::ctrl_c().await.unwrap();
}