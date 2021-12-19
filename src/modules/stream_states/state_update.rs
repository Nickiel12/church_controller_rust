use super::enums::{SubScenes, Scenes};
use serde_json::Value;


#[derive(Debug, PartialEq, Clone)]
pub enum StateUpdate {
    StreamRunning(bool),
    StreamIsMuted(bool),
    ComputerSoundIsOn(bool),
    ChangeSceneOnChangeSlideHotkey(bool),
    SceneIsAugmented(bool),
    TimerCanRun(bool),
    TimerLength(f32),
    TimerText(String),
    SubScene(SubScenes),
    Scene(Scenes),
    UpdateClient,
}

impl StateUpdate {
    pub fn json_to_state_update(incoming_json: Value) -> Self {
        let message_type = &incoming_json["type"];

        match message_type.as_str().unwrap() {
            "button" => {
                let value = &incoming_json["button"];
                match value.as_str().unwrap() {
                    "Scene_Camera" => {StateUpdate::Scene(Scenes::Camera)}

                    _ => {panic!("trying to use a button type I don't know!")}
                }
            },
            "Timer_Length" => {
                let new_timer_length = &incoming_json["data"];
                StateUpdate::TimerLength(new_timer_length.as_f64().unwrap() as f32)
            },
            "update" => {
                StateUpdate::UpdateClient
            },
            _ => {
                panic!("State Update Could Not Cast the json: {:?}", incoming_json.as_str());
            }
        }

    }
}