use crossbeam_channel::unbounded;
use std::io::Write;
use std::thread;
use std::time::Duration;

use crate::modules::socket_handler::Socket;

#[test]
fn can_make_socket_listener() {
    let listener = Socket::make_listener("localhost:5001");
    drop(listener);
}

#[test]
fn create_and_connect_to_listener() {
    let listener = Socket::make_listener("localhost:5002");

    let join_handle = std::thread::spawn(move || {
        let _outgoing = std::net::TcpStream::connect("localhost:5002").unwrap();
    });
    join_handle.join().unwrap();
    drop(listener);
}

#[test]
#[should_panic]
fn panic_no_listener() {
    let _outgoing = std::net::TcpStream::connect("localhost:5003").unwrap();
}

#[test]
fn can_handle_messages() {
    let listener = Socket::make_listener("localhost:5004");
    let (tx_1, rx_1) = unbounded::<String>();

    let mut socket = Socket::handle_connections(listener, tx_1);

    let join_handle = std::thread::spawn(move || {
        let mut outgoing = std::net::TcpStream::connect("localhost:5004").unwrap();
        outgoing.write("this is a test".as_bytes()).unwrap();
        drop(outgoing);
    });
    join_handle.join().unwrap();
    thread::sleep(Duration::from_millis(1000));

    let message = rx_1.recv().unwrap();
    assert_eq!(message, String::from("this is a test"));

    socket.close();
}

#[test]
fn can_handle_delayed_messages() {
    let listener = Socket::make_listener("localhost:5005");
    let (tx_1, rx_1) = unbounded::<String>();

    let mut socket = Socket::handle_connections(listener, tx_1);

    let mut outgoing = std::net::TcpStream::connect("localhost:5005").unwrap();
    outgoing.write("this is a test1\n".as_bytes()).unwrap();
    thread::sleep(Duration::from_millis(500));
    outgoing.write("this is a test3\n".as_bytes()).unwrap();
    drop(outgoing);
    thread::sleep(Duration::from_millis(500));

    let message = rx_1.recv().unwrap();
    println!("{}", message);
    assert_eq!(message, String::from("this is a test1\n"));

    let message = rx_1.recv().unwrap();
    println!("{}", message);
    assert_eq!(message, String::from("this is a test3\n"));

    socket.close();
}
