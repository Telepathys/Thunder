use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    sync::{Mutex},
};
use tokio::net::{TcpListener};
use log::{info,error};
use tonic::{transport::Server};
use thunder::{hello_server};

// Custom use
pub mod socket;
mod grpc {
    pub mod thunder {
        pub mod test {
            pub mod hello_service;
        }
    }
}
use socket::{
    handle_connection,
    TotalSocket,
};
use grpc::thunder::test::hello_service::{
    HelloService, thunder,
};

#[tokio::main]
async fn main() -> Result<(), IoError> {
    // logger level setting
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    // socket server start
    let addr = env::args().nth(1).unwrap_or_else(|| "0.0.0.0:7777".to_string());
    let total_socket = TotalSocket::new(Mutex::new(HashMap::new()));
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    tokio::spawn(async move {
        info!("socket server start : {}", addr);
        // socket main loop
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(handle_connection(total_socket.clone(),stream, addr));
        }
    });

    // gRPC server start
    let hello_service = HelloService::default();
    let addr = "[::1]:7778".parse().unwrap();
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

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}