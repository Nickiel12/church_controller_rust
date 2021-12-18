use std::sync::mpsc;
use std::io::{Write};
use std::thread;
use std::time::Duration;

use crate::{SERVER_ADDRESS, modules::socket_handler::Socket};



#[test]
fn can_make_socket_listener(){
    let listener = Socket::make_listener(SERVER_ADDRESS);
    drop(listener);
}

#[test]
fn create_and_connect_to_listener() {
    let listener = Socket::make_listener(SERVER_ADDRESS);

    let join_handle = std::thread::spawn(move || {
        let _outgoing = std::net::TcpStream::connect(SERVER_ADDRESS).unwrap();
    });
    join_handle.join().unwrap();
    drop(listener);
}

#[test]
#[should_panic]
fn panic_no_listener() {
    let _outgoing = std::net::TcpStream::connect("localhost").unwrap();
}

#[test]
fn can_handle_messages() {
    let listener = Socket::make_listener(SERVER_ADDRESS);
    let (tx_1, rx_1) = mpsc::channel::<String>();

    let (mut flag, connection_handle) = Socket::handle_connections(listener, tx_1);

    let join_handle = std::thread::spawn(move || {
        let mut outgoing = std::net::TcpStream::connect(SERVER_ADDRESS).unwrap();
        outgoing.write("this is a test".as_bytes()).unwrap();
    });
    join_handle.join().unwrap();
    thread::sleep(Duration::from_millis(1000));

    flag.set(false);
    connection_handle.join().unwrap();
    let message = rx_1.recv().unwrap();
    assert_eq!(message, String::from("this is a test"));
}
