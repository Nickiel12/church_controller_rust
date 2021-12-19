use super::stream_states::{state_update::StateUpdate, stream_states_class::StreamState};

pub enum StateMessage {
    StateUpdate(StateUpdate),
    GetStates,
    CloseListener,
}

pub trait MessageHandler {
    fn handle_update(&mut self, update: StateUpdate) -> ();
    fn create_update_from_string(update_json: String) -> StateUpdate;
    fn get_states(&self) -> StreamState;
}

impl MessageHandler for StreamState {
    fn handle_update(&mut self, update: StateUpdate) {
        self.update(update.clone());
    }   
    fn get_states(&self) -> StreamState{
        self.clone()
    }
    fn create_update_from_string(update_json: String) -> StateUpdate {
        let json: serde_json::Value = serde_json::from_str(&update_json[1..]).unwrap();
        let message_type = &json["type"];

        match message_type.as_str().unwrap() {
            "button" => {
                let value = &json["button"];
            },
            "Timer_Length" => {
                let new_timer_length = &json["data"];
            },
            "update" => {
                println!("Update all!! *Poof*!");
            },
            _ => {

            }
        }

        println!("type: {}", json["type"]);
        StateUpdate::ChangeSceneOnChangeSlideHotkey(false)
    }
}