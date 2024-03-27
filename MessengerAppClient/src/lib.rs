use lazy_static::lazy_static;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::net::{TcpStream};

pub mod messages;

pub const IP_ADDRESS:&str = "192.168.5.248";
pub const PORT_NUMBER:&str = "8888";

// This is the connection to the database for the entire program.
// There should never be a new connection made, this will cause the information
// from the past connection to no longer persist.
lazy_static! {
    pub static ref ADDRESS: Arc<Mutex<String>> = Arc::new(
    Mutex::new(
    format!(
    "{}:{}", IP_ADDRESS, PORT_NUMBER)));
}

lazy_static! {
    pub static ref CONN: Arc<Mutex<Connection>> =
    Arc::new(
    Mutex::new(
    Connection::open_in_memory()
    .expect("Could not create Connection.")));
}