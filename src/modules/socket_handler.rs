use workctl::sync_flag;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread::{self, JoinHandle};
use std::time::Duration;



trait SocketCallback {
    fn handle_message(message: String);
}

pub struct Socket{

}

impl Socket {

    pub fn make_listener(address: &str) -> TcpListener {
        TcpListener::bind(address).unwrap()
    }

    pub fn handle_connections(listener: TcpListener) -> (sync_flag::SyncFlagTx, JoinHandle<()>){
        let (tx, thread_stop_flag) = sync_flag::new_syncflag(true);
        let handle = thread::spawn(move || {
            while thread_stop_flag.get() {
                for (stream, addr) in listener.accept() {
                    
                }
                thread::sleep(Duration::from_millis(100));
            }
            drop(listener);
        });
        (tx, handle)
    }

    pub fn handle_client(mut stream: TcpStream) {

        let mut buffer = [0; 1024];

        let read_size = stream.read(&mut buffer).unwrap();

        if read_size == 0 {
            stream.shutdown(Shutdown::Both).unwrap();
            return
        }

        let output = String::from_utf8_lossy(&buffer[..]);
        println!("recieved: {}", output);
    }
}