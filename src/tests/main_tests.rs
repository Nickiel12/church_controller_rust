use crate::{make_listener, SERVER_ADDRESS};


#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn can_make_socket_listener(){
    let listener = make_listener(SERVER_ADDRESS);
    drop(listener);
}

#[test]
fn create_and_connect_to_listener() {
    let listener = make_listener(SERVER_ADDRESS);

    let join_handle = std::thread::spawn(move || {
        let _outgoing = std::net::TcpStream::connect(SERVER_ADDRESS).unwrap();
    });
    join_handle.join().unwrap();
    drop(listener);
}

#[test]
#[should_panic]
fn panic_no_listener() {
    let _outgoing = std::net::TcpStream::connect(SERVER_ADDRESS).unwrap();
}

