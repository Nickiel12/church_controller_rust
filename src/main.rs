use std::{sync::mpsc, time::Duration};

use modules::{socket_handler::Socket, stream_states::stream_states_class::StreamState, message_handler::{MessageHandler, StateMessage}};
use workctl::sync_flag;

mod tests;
mod modules;



const SERVER_ADDRESS: &str = "10.0.0.168:5000";

fn main() {
    let state = StreamState::new();

    let socket_listener = Socket::make_listener(SERVER_ADDRESS);
    let (from_socket_tx, from_socket_rx) = mpsc::channel::<String>();
    let (mut listener_can_run_flag, listener_join_handle) = Socket::handle_connections(socket_listener, from_socket_tx);
    
    let (control_c_flag_tx, control_c_called_flag_rx) = sync_flag::new_syncflag(false);
    
    setup_control_c(control_c_flag_tx);
    
    //until control_c is caught, check the queue of incoming
    //requests from the socket handler.
    while !control_c_called_flag_rx.get() {
        match from_socket_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(message) => {
                println!("{}", message);
                let update = <StreamState as MessageHandler>::create_update_from_string(message);
            },
            Err(_) => {continue},
        }
    }
    
    //Close the listener thread
    listener_can_run_flag.set(false);
    listener_join_handle.join().unwrap();
}

fn setup_control_c(mut control_c_flag_tx: sync_flag::SyncFlagTx) {
    ctrlc::set_handler(move || {
        control_c_flag_tx.set(true);
    }).expect("control C handler failed!");
}

