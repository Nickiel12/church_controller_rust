use super::stream_states::{state_update::StateUpdate, stream_states_class::StreamState};

pub enum StateMessage {
    StateUpdate(StateUpdate),
    GetStates,
    CloseListener,
}

pub trait MessageHandler {
    fn handle_update(&mut self, update: StateUpdate) -> ();
    fn get_states(&self) -> StreamState;
}

impl MessageHandler for StreamState {
    fn handle_update(&mut self, update: StateUpdate) {
        self.update(update.clone());
    }   
    fn get_states(&self) -> StreamState{
        self.clone()
    }

}