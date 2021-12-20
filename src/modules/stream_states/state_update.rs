use std::ops::Add;

use super::enums::{SubScenes, Scenes};
use serde_json::Value;


#[derive(Debug, PartialEq, Clone)]
pub enum StateUpdate {
    StreamRunning(bool),
    StreamIsMuted(bool),
    ComputerSoundIsOn(bool),
    ComputerMediaDoPause(bool),
    ChangeSceneOnChangeSlide(bool),
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
                    //Master Scenes
                    "Scene_Camera" => {StateUpdate::Scene(Scenes::Camera)}
                    "Scene_Screen" => {StateUpdate::Scene(Scenes::Screen)}
                    "Scene_Is_Augmented"    => {StateUpdate::SceneIsAugmented(incoming_json["data"].as_bool().unwrap())},
                    
                    //Slide changing behavior
                    "Timer_Can_Run" => {StateUpdate::TimerCanRun(incoming_json["data"].as_bool().unwrap())}
                    "Change_With_Clicker" => {StateUpdate::ChangeSceneOnChangeSlide(incoming_json["data"].as_bool().unwrap())},
                    
                    //Extra Toggles
                    "Toggle_Computer_Volume" => {StateUpdate::ComputerSoundIsOn(incoming_json["data"].as_bool().unwrap())},
                    "Toggle_Stream_Volume" => {StateUpdate::StreamIsMuted(incoming_json["data"].as_bool().unwrap())},
                    "Media_Pause_Play" => {StateUpdate::ComputerMediaDoPause(incoming_json["data"].as_bool().unwrap())},

                    //SubScenes
                    "Camera_None" => {StateUpdate::SubScene(SubScenes::CameraDefault)},
                    "Camera_Top_Right" => {StateUpdate::SubScene(SubScenes::CameraWithUpperRight)},
                    "Camera_Bottom_Right" => {StateUpdate::SubScene(SubScenes::CameraWithLowerRight)},
                    "Camera_Bottom_Left" => {StateUpdate::SubScene(SubScenes::CameraWithLargeUpperRight)},
                    "Screen_None" => {StateUpdate::SubScene(SubScenes::ScreenDefault)},
                    "Screen_Top_Right" => {StateUpdate::SubScene(SubScenes::ScreenWithUpperRight)},
                    "Screen_Bottom_Right" => {StateUpdate::SubScene(SubScenes::ScreenWithLowerRight)},
                    
                    //Unimplemented
                    "Next_Slide" |
                    "Prev_Slide" |
                    _ => {panic!("trying to use a button type I don't know!: {}", value)}
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

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            StateUpdate::StreamRunning(is_true) => {
                serde_json::json!({
                "type": "button",
                "button": "Stream_Running",
                "data": is_true
            })},
            StateUpdate::StreamIsMuted(is_true) => {
                serde_json::json!({
                    "type": "button",
                    "button": "Stream_Is_Muted",
                    "data": is_true
            })},
            StateUpdate::ComputerSoundIsOn(is_true) => {
                serde_json::json!({
                    "type": "button",
                    "button": "Computer_Sound_Is_On",
                    "data": is_true,
            })},
            StateUpdate::ChangeSceneOnChangeSlide(is_true) => {
                serde_json::json!({
                    "type": "button",
                    "button": "Change_With_Clicker",
                    "data": is_true,
            })},
            StateUpdate::SceneIsAugmented(is_true) => {
                serde_json::json!({
                    "type": "button",
                    "type": "Scene_Is_Augmented",
                    "data": is_true,
            })},
            StateUpdate::TimerCanRun(is_true) => {
                serde_json::json!({
                    "type": "button",
                    "button": "Timer_Can_Run",
                    "data": is_true,
            })},
            StateUpdate::TimerLength(length) => {
                serde_json::json!({
                    "type": "button",
                    "button": "Timer_Length",
                    "data": length,
            })},
            StateUpdate::TimerText(text) => {
                serde_json::json!({
                    "type": "button",
                    "button": "Timer_Text",
                    "data": text,
            })},
            StateUpdate::SubScene(scene) => {
                serde_json::json!({
                    "type": "button",
                    "button": "SubScene",
                    "SubScene": scene.to_string(),
            })},
            StateUpdate::Scene(scene) => {
                serde_json::json!({
                    "type": "button",
                    "button": "Scene",
                    "Scene": scene.to_string(),
            })},
            StateUpdate::ComputerMediaDoPause(is_true) => todo!(),
            StateUpdate::UpdateClient => todo!(),
        }
    }
}