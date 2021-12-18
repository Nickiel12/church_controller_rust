use std::sync::mpsc;
use std::io::{Write};
use std::thread;
use std::time::Duration;

use modules::socket_handler::Socket;


mod tests;
mod modules;



const SERVER_ADDRESS: &str = "localhost:5000";

fn main() {
    println!("hello world");
}
