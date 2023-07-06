use std::{
    io::Error as IoError,
};

pub mod core;
pub mod socket;
pub mod router {
    pub mod server{
        pub mod http_router;
        pub mod socket_router;
    }
    pub mod service {
        pub mod message_router;
        pub mod group_router;
        pub mod match_router;
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
        pub mod message {
            pub mod message_hash;
        }
        pub mod matchs {
            pub mod match_hash;
        }
    }
}
pub mod utils {
    pub mod sha;
    pub mod jwt;
}
// Game Module
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
        pub mod matchs {
            pub mod random_match_wait_join_component;
            pub mod random_match_join_component;
            pub mod random_match_wait_success_componet;
            pub mod random_match_complete_component;
        }
        pub mod config {
            pub mod config_component;
        }
    }
    pub mod systems {
        pub mod message {
            pub mod system_message_system;
            pub mod server_message_system;
            pub mod whisper_message_system;
            pub mod group_message_system;
            pub mod message_limit_system;
        }
        pub mod group {
            pub mod group_join_system;
            pub mod group_leave_system;
        }
        pub mod matchs {
            pub mod random_match_wait_system;
            pub mod random_match_cancel_system;
            pub mod random_match_wait_join_system;
            pub mod random_match_join_system;
            pub mod random_match_complete_system;
        }
    }
    pub mod memory {
        pub mod config {
            pub mod config_memory;
        }
        pub mod user {
            pub mod user_memory;
        }
    }
    pub mod enums {
        pub mod core_enum;
    }
    pub mod scheduler {
        pub mod scheduler;
        pub mod matchs {
            pub mod random_match_scheduler;
        }
    }
}
use game::memory::config::config_memory::config_init;

use crate::core::server_start;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    if config_init().await {
        server_start().await;
    }
    Ok(())
}