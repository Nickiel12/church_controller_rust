use std::{sync::mpsc::{Sender, self, Receiver}, thread::{JoinHandle, self}};

use super::{message_handler::{MessageHandler, StateMessage}, stream_states::{stream_states_class::StreamState}};


pub struct StatesIOHandler {
    pub listener_join_handler: Option<JoinHandle<()>>,
    pub message_thread_tx: Option<Sender<StateMessage>>,
    data_rx: Option<Receiver<StreamState>>,
}

impl StatesIOHandler {
    pub fn new() -> Self {
        StatesIOHandler{
            ..Default::default()
        }
    }

    pub fn start_socket_listener(&mut self, mut message_handler: impl MessageHandler + Send + 'static) {
        let (tx, rx) = mpsc::channel::<StateMessage>();
        let (tx_1, rx_1) = mpsc::channel::<StreamState>();
        self.message_thread_tx = Some(tx);
        self.data_rx = Some(rx_1);

        let handle = thread::spawn(move || {
            for message in rx {
                match message {
                    StateMessage::StateUpdate(message) => {
                        message_handler.handle_update(message);
                    },
                    StateMessage::GetStates => {
                        tx_1.send(message_handler.get_states()).unwrap();
                    }
                    StateMessage::CloseListener => {
                        tx_1.send(message_handler.get_states()).unwrap();
                        break;
                    }
                }
            }
            
        });
        self.listener_join_handler = Some(handle);
    }

    pub fn get_states(&self) -> StreamState {
        let tx = self.message_thread_tx.clone();
        match tx {
            Some(tx) => {
                tx.send(StateMessage::GetStates).unwrap();
                let rx = self.data_rx.as_ref();
                match rx {
                    Some(rx) => {
                        let message = rx.recv().unwrap();
                        message
                    },
                    None => {panic!("Trying to get data_rx before IOHandler has it!");}
                }
            },
            None => {panic!("trying to access states before IOHandler has a handle for transmitting");}
        }
    }

    pub fn close(self) -> StreamState {
        let state = self.get_states();
        assert_eq!(self.listener_join_handler.and_then(|f| {
                let tx = self.message_thread_tx.clone().unwrap();
                tx.send(StateMessage::CloseListener).unwrap();
                f.join().unwrap();
                Some(true)
            }).unwrap(), true);
        state
    }
}

impl Default for StatesIOHandler {
    fn default() -> Self {
        StatesIOHandler {
            message_thread_tx: None,
            listener_join_handler: None,
            data_rx: None,
        }
    }
}