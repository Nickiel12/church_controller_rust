use crate::make_listener;


#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn can_make_socket_listener(){
    let listener = make_listener(String::from("localhost:5000"));
    drop(listener);
}

#[test]
fn create_and_connect_to_listener() {
    let listener = make_listener(String::from("localhost:5000"));

    let join_handle = std::thread::spawn(move || {
        let _outgoing = std::net::TcpStream::connect("localhost:5000").unwrap();
    });
    join_handle.join().unwrap();
    drop(listener);
}

