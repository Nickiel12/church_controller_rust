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