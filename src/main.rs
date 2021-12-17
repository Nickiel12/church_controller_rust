
mod tests;
mod modules;

use std::net::{TcpListener, TcpStream, Shutdown};

const SERVER_ADDRESS: &str = "localhost:5000";

fn main() {
    let listener = make_listener(SERVER_ADDRESS);



    drop(listener);
}

fn make_listener(address: &str) -> TcpListener {
    TcpListener::bind(address).unwrap()
}
