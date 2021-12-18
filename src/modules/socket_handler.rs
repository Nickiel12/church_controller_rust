use workctl::sync_flag;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::message_handler::StateMessage;

trait SocketCallback {
    fn handle_message(message: String);
}

pub struct Socket{

}

impl Socket {

    pub fn make_listener(address: &str) -> TcpListener {
        TcpListener::bind(address).unwrap()
    }

    pub fn handle_connections(listener: TcpListener, update_tx: Sender<String>) -> (sync_flag::SyncFlagTx, JoinHandle<()>){
        let (tx, thread_stop_flag) = sync_flag::new_syncflag(true);
        let handle = thread::spawn(move || {
            listener.set_nonblocking(true).unwrap();
            while thread_stop_flag.get() {
                for (stream, addr) in listener.accept() {
                    Socket::handle_client(stream, update_tx.clone());
                }
                thread::sleep(Duration::from_millis(100));
            }
            drop(listener);
        });
        (tx, handle)
    }

    pub fn handle_client(mut stream: TcpStream, update_tx: Sender<String>) {

        let mut buffer = [0; 1024]; 

        let read_size = stream.read(&mut buffer).unwrap();

        if read_size == 0 {
            stream.shutdown(Shutdown::Both).unwrap();
            return
        }

        let output = String::from_utf8_lossy(&buffer[0..read_size]);
        println!("recieved: {}", output);
        update_tx.send(output.into_owned()).unwrap();
    }
}