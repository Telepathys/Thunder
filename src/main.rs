use std::{
    env,
    io::Error as IoError,
};
use database::redis::connect::{redis_subscribe};
use tokio::net::{TcpListener};
use log::{info,error};
use tonic::{transport::Server};
use thunder::{hello_server};
use actix_web::{App, HttpServer};
use dotenv::dotenv;

// Custom use
pub mod socket;
pub mod router {
    pub mod server{
        pub mod http_router;
        pub mod socket_router;
    }
    pub mod service {
        pub mod message_router;
        pub mod group_router;
    }
}
mod grpc {
    pub mod thunder {
        pub mod test {
            pub mod hello_service;
        }
    }
}
pub mod database {
    pub mod mongo {
        pub mod user{
            pub mod users;
        }
        pub mod connect;
    }
    pub mod redis {
        pub mod connect;
        pub mod test {
            pub mod test;
        }
        pub mod socket {
            pub mod socket_hash;
        }
        pub mod group {
            pub mod group_hash;
        }
    }
}
pub mod utils {
    pub mod sha;
    pub mod jwt;
}
pub mod game {
    pub mod components {
        pub mod user {
            pub mod user_component;
        }
        pub mod message {
            pub mod system_message_component;
            pub mod server_message_component;
            pub mod whisper_message_component;
            pub mod group_message_component;
        }
        pub mod redis {
            pub mod redis_component;
        }
        pub mod group {
            pub mod group_join_component;
            pub mod group_leave_component;
        }
    }
    pub mod systems {
        pub mod message {
            pub mod system_message_system;
            pub mod server_message_system;
            pub mod whisper_message_system;
            pub mod group_message_system;
        }
        pub mod group {
            pub mod group_join_system;
            pub mod group_leave_system;
        }
    }
    pub mod memory {
        pub mod user {
            pub mod user_memory;
        }
    }
    pub mod enums {
        pub mod core_enum;
    }
}
use router::server::http_router::{
    join,
    login,
    asd,
};
use socket::{
    handle_connection,
};
use grpc::thunder::test::hello_service::{
    HelloService, thunder,
};

#[tokio::main]
async fn main() -> Result<(), IoError> {
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
    .bind(&http_addr)?
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

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}