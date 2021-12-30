use workctl::sync_flag;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::{Mutex, Arc};
use crossbeam_channel::{Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct Socket{
    socket_txs: Arc<Mutex<Vec<Arc<TcpStream>>>>,
    stop_listener_flag: sync_flag::SyncFlagTx,
    handle_connections_join_handle: Option<JoinHandle<()>>,
    pub has_closed: workctl::sync_flag::SyncFlagRx,
}

impl Socket {

    pub fn make_listener(address: &str) -> TcpListener {
        TcpListener::bind(address).unwrap()
    }

    pub fn handle_connections(listener: TcpListener, messenger_tx: Sender<String>) -> Self {
        let (mut has_closed_tx, has_closed_rx) = sync_flag::new_syncflag(false);
        let (tx, thread_stop_flag) = sync_flag::new_syncflag(true);
        let socket_streams = Arc::new(Mutex::new(vec![]));

        let thread_owned_streams = Arc::clone(&socket_streams);
        let handle = thread::spawn(move || {
            listener.set_nonblocking(true).unwrap();
            while thread_stop_flag.get() {
                for (strm, _addr) in listener.accept() {
                    //create thread-passable pointer of current socket stream
                    let stream = Arc::new(strm);
                    //get lock on list of thread-passable pointers
                    let mut streams = thread_owned_streams.lock().unwrap();
                    streams.push(Arc::clone(&stream));
                    //pass off a clone of the thread-passable pointer
                    drop(streams);
                    Socket::handle_client(Arc::clone(&stream), messenger_tx.clone(),  thread_stop_flag.clone());
                }
                thread::sleep(Duration::from_millis(100));
            }
            has_closed_tx.set(true);
            drop(listener);
        });
        Socket {
            socket_txs: socket_streams,
            stop_listener_flag: tx,
            handle_connections_join_handle: Some(handle),
            has_closed: has_closed_rx
        }
    }

    pub fn handle_client(stream: Arc<TcpStream>, update_tx: Sender<String>, program_shutdown_flag: sync_flag::SyncFlagRx) {
        let mut buffer = [0; 1024];
        stream.set_read_timeout(Some(Duration::from_millis(100))).expect("Could not set a read timeout");
        while program_shutdown_flag.get() {
            match stream.as_ref().read(&mut buffer) {
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
        }
        stream.shutdown(Shutdown::Both).unwrap();
    }

    pub fn close(&mut self) {
        self.stop_listener_flag.set(false);
        self.handle_connections_join_handle
            .take().expect("Called on not running thread")
            .join().expect("Could not join thread");
    }

    pub fn send(&self, message: String) {
        let mut streams = self.socket_txs.lock().unwrap();
        for i in 0..streams.len(){
            let mut tx = streams.get(i).unwrap().as_ref();
            
            match tx.write(message.clone().as_bytes()) {
                Err(_) => {streams.remove(i); continue;},
                Ok(_) => {},
            }
            tx.flush().unwrap();
            println!("sent");
        }
    }
}