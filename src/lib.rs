#![deny(missing_docs)]
//! 主程式會用的的各種功能

mod msg_db;
pub use msg_db::*;

pub mod prelude;

mod rpc;
pub use rpc::Server;

#[allow(missing_docs)]
pub mod proto {
    tonic::include_proto!("chat");
}
