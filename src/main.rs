use std::sync::mpsc;

use modules::socket_handler::Socket;

mod tests;
mod modules;



const SERVER_ADDRESS: &str = "10.0.0.168:5000";

fn main() {
    let socket_listener = Socket::make_listener(SERVER_ADDRESS);
    let (from_socket_tx, from_socket_rx) = mpsc::channel::<String>();
    let (mut listener_flag, listener_join_handle) = Socket::handle_connections(socket_listener, from_socket_tx);

    let message = from_socket_rx.recv().unwrap();
    println!("{}", message);

    listener_flag.set(false);
    listener_join_handle.join().unwrap();
}
