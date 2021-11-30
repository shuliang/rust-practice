// #![allow(unused)]

pub mod client;

pub mod cmd;
pub use cmd::Command;

mod connection;
pub use connection::Connection;

mod frame;
pub use frame::Frame;

mod db;
use db::Db;

mod parse;
use parse::Parse;

pub mod server;

mod buffer;
pub use buffer::{buffer, Buffer};

mod shutdown;
use shutdown::Shutdown;

pub const DEFAULT_PORT: &str = "6379";

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
