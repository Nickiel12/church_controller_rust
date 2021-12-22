use workctl::sync_flag;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use crossbeam_channel::{Sender, Receiver};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct Socket{
    socket_txs: Vec<Sender<String>>,
    stop_listener_flag: sync_flag::SyncFlagTx,
    handle_connections_join_handle: Option<JoinHandle<()>>,
}

impl Socket {

    pub fn make_listener(address: &str) -> TcpListener {
        TcpListener::bind(address).unwrap()
    }

    pub fn handle_connections(listener: TcpListener, messenger_tx: Sender<String>, messenger_rx: Receiver<String>) -> Self {
        let (tx, thread_stop_flag) = sync_flag::new_syncflag(true);
        
        let handle = thread::spawn(move || {
            listener.set_nonblocking(true).unwrap();
            while thread_stop_flag.get() {
                for (stream, _addr) in listener.accept() {
                    Socket::handle_client(stream, messenger_tx.clone(), messenger_rx.clone(),  thread_stop_flag.clone());
                }
                thread::sleep(Duration::from_millis(100));
            }
            drop(listener);
        });
        Socket {
            socket_txs: Vec::<Sender<String>>::new(),
            stop_listener_flag: tx,
            handle_connections_join_handle: Some(handle),
        }
    }

    pub fn handle_client(mut stream: TcpStream, update_tx: Sender<String>, message_rx: Receiver<String>, program_shutdown_flag: sync_flag::SyncFlagRx) {
        let mut buffer = [0; 1024];
        stream.set_read_timeout(Some(Duration::from_millis(100))).expect("Could not set a read timeout");
        while program_shutdown_flag.get() {
            match stream.read(&mut buffer) {
                Err(_) => {},
                Ok(read_size) => {
                    //Tcp is supposed to have a 0 byte read if closed by client
                    if read_size == 0 || !program_shutdown_flag.get() {
                        break;
        
                    } else {
                        let output = String::from_utf8_lossy(&buffer[0..read_size]);
                        update_tx.send(output.into_owned()).unwrap();
                    }
                }
            }
            match message_rx.try_recv() {
                Err(_) => {},
                Ok(message) => {
                    stream.write(message.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }
        }
        stream.shutdown(Shutdown::Both).unwrap();
    }

    pub fn close(&mut self) {
        self.stop_listener_flag.set(false);
        self.handle_connections_join_handle
            .take().expect("Called on not running thread")
            .join().expect("Could not join thread");
    }
}