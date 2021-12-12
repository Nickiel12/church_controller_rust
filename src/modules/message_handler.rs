use std::sync::mpsc::{Receiver};

use super::stream_states::{stream_states_class::StreamStates, enums::StateUpdate};

pub enum StateMessage {
    StateUpdateContainer(StateUpdate),
    CloseListener,
}

pub trait MessageListnerThread {
    fn listen(self, rx: Receiver<StateMessage>) -> std::thread::JoinHandle<()>;
}

#[derive(Debug)]
pub struct MessageHandler {
    pub state: StreamStates,
}

impl MessageHandler {
    pub fn new() -> Self {
        return MessageHandler {
            state: StreamStates::new(),
        }
    }

    pub fn handle_update(&mut self, update: StateUpdate) {
        self.state.update(update.clone());
    }

    
}