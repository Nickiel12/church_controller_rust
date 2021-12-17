
mod tests;
mod modules;

use std::net::{TcpListener, TcpStream, Shutdown};

fn main() {
    let listener = make_listener(String::from("localhost:5000"));



    drop(listener);
}

fn make_listener(address: String) -> TcpListener {
    TcpListener::bind(address).unwrap()
}
