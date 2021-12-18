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

    let (mut flag, connection_handle) = Socket::handle_connections(listener);

    let join_handle = std::thread::spawn(move || {
        let _outgoing = std::net::TcpStream::connect(SERVER_ADDRESS).unwrap();
    });
    join_handle.join().unwrap();
    flag.set(false);
    connection_handle.join().unwrap();
}
