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
}

impl Socket {

    pub fn make_listener(address: &str) -> TcpListener {
        TcpListener::bind(address).unwrap()
    }

    pub fn handle_connections(listener: TcpListener, messenger_tx: Sender<String>) -> Self {
        let (tx, thread_stop_flag) = sync_flag::new_syncflag(true);
        let socket_streams = Arc::new(Mutex::new(vec![]));

        let thread_owned_streams = Arc::clone(&socket_streams);

        println!("initializing socket connection handling thread");
        let handle = thread::spawn(move || {
            listener.set_nonblocking(true).unwrap();
            let mut service_sockets: Vec<Arc<TcpStream>> = Vec::new();
            while thread_stop_flag.get() {
                for (s, _addr) in listener.accept() {
                    s.set_nonblocking(true).unwrap();
                    let stream = Arc::new(s);
                    let mut streams = thread_owned_streams.lock().unwrap();
                    streams.push(Arc::clone(&stream));
                    service_sockets.push(Arc::clone(&stream));
                    drop(streams);
                }
                Socket::service_clients(&mut service_sockets, messenger_tx.clone());
                thread::sleep(Duration::from_millis(100));
            }
            println!("closed socket connection handling thread");
            drop(listener);
        });

        Socket {
            socket_txs: socket_streams,
            stop_listener_flag: tx,
            handle_connections_join_handle: Some(handle),
        }
    }

    pub fn service_clients(streams: &mut Vec<Arc<TcpStream>>, update_tx: Sender<String>) {
        let mut buffer = [0; 1024];
        let mut remove = Vec::new();
        for i in 0..streams.len() {
            let resp = streams.get(i).as_ref().unwrap().as_ref().read(&mut buffer);
            if resp.is_ok() {
                let msg_len = resp.unwrap();
                if msg_len == 0 {
                    remove.push(i);
                } else {
                    update_tx.send(String::from_utf8_lossy(&buffer[0..msg_len]).into_owned()).unwrap();
                }
            }
        }
        for i in remove.iter() {
            streams.get(*i).unwrap().shutdown(Shutdown::Both).unwrap();
            streams.remove(*i);
        }

    }

    pub fn close(&mut self) {
        self.stop_listener_flag.set(false);
        self.handle_connections_join_handle
            .take().expect("Called on not running thread")
            .join().expect("Could not join thread");
    }

    pub fn send(&self, message: String) {
        let mut streams = self.socket_txs.lock().unwrap();
        let mut removes = Vec::<usize>::new();
        if streams.len() == 0 {return}
        for i in 0..streams.len(){
            match streams.get(i) {
                None => {
                    removes.push(i);
                }, 
                Some(strm) => {
                    let mut tx = strm.as_ref();
                    match tx.write(message.clone().as_bytes()) {
                        Err(_) => {
                            removes.push(i)
                        },
                        Ok(_) => {
                            tx.write(b"\n").unwrap();
                            tx.flush().unwrap();
                        }
                    }
                }
            }
        }
        for i in removes.iter() {
            streams.remove(*i);
        }
    }
}