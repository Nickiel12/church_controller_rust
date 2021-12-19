use std::{sync::mpsc, time::Duration};

use modules::socket_handler::Socket;
use workctl::sync_flag;

mod tests;
mod modules;



const SERVER_ADDRESS: &str = "10.0.0.168:5000";

fn main() {
    let socket_listener = Socket::make_listener(SERVER_ADDRESS);
    let (from_socket_tx, from_socket_rx) = mpsc::channel::<String>();
    let (mut listener_can_run_flag, listener_join_handle) = Socket::handle_connections(socket_listener, from_socket_tx);
    
    let (mut control_c_flag_tx, control_c_called_flag_rx) = sync_flag::new_syncflag(false);
    
    ctrlc::set_handler(move || {
        control_c_flag_tx.set(true);
    }).expect("control C handler failed!");
    
    while !control_c_called_flag_rx.get() {
        match from_socket_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(message) => {
                println!("{}", message);
            },
            Err(_) => {continue},
        }
    }
    
    println!("ding");
    listener_can_run_flag.set(false);
    listener_join_handle.join().unwrap();
    }
