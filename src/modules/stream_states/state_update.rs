
use super::scenes::{SubScenes, Scenes, SlideChange};
use serde_json::Value;


#[derive(Debug, PartialEq, Clone)]
pub enum StateUpdate {
    StreamRunning(bool),
    StreamSoundToggleOn(bool),
    ToggleComputerSoundOn(bool),
    ComputerMediaDoPause,
    ChangeSceneOnChangeSlide(bool),
    SceneIsAugmented(bool),
    TimerCanRun(bool),
    PauseTimer(bool),
    TimerLength(f32),
    TimerText(String),
    SubScene(SubScenes),
    Scene(Scenes),
    ChangeSlide(SlideChange),
    UpdateClient,
}

impl StateUpdate {
    pub fn json_to_state_update(incoming_json: Value) -> Self {

        match incoming_json["type"].as_str().unwrap() {
            "update" => {
                let value = &incoming_json["update"];
                match value.as_str().unwrap() {
                    //Master Scenes
                    "Scene" => {
                        let scene = incoming_json["data"].as_str().unwrap();
                        match scene {
                            "Scene_Camera" => {StateUpdate::Scene(Scenes::Camera)}
                            "Scene_Screen" => {StateUpdate::Scene(Scenes::Screen)}
                            _ => {panic!("unknown Scene! {}", scene)}                            
                        }
                    }

                    "Scene_Is_Augmented" => {StateUpdate::SceneIsAugmented(string_to_bool(incoming_json["data"].as_str().unwrap()))},

                    //SubScenes
                    "SubScene" => {
                        let subscene = incoming_json["data"].as_str().unwrap();
                        match subscene {
                            "Camera_None" => {StateUpdate::SubScene(SubScenes::CameraDefault)},
                            "Camera_Top_Right" => {StateUpdate::SubScene(SubScenes::CameraWithUpperRight)},
                            "Camera_Bottom_Right" => {StateUpdate::SubScene(SubScenes::CameraWithLowerRight)},
                            "Camera_Bottom_Left" => {StateUpdate::SubScene(SubScenes::CameraWithLargeUpperRight)},
                            "Screen_None" => {StateUpdate::SubScene(SubScenes::ScreenDefault)},
                            "Screen_Top_Right" => {StateUpdate::SubScene(SubScenes::ScreenWithUpperRight)},
                            "Screen_Bottom_Right" => {StateUpdate::SubScene(SubScenes::ScreenWithLowerRight)},
                            _ => {panic!("unknown SubScene! {}", subscene)}
                        }
                    }
                    
                    //Slide changing behavior
                    "Timer_Can_Run" => {StateUpdate::TimerCanRun(string_to_bool(incoming_json["data"].as_str().unwrap()))}
                    "Change_With_Clicker" => {StateUpdate::ChangeSceneOnChangeSlide(string_to_bool(incoming_json["data"].as_str().unwrap()))},
                    "Timer_Length" => {
                        let new_timer_length = &incoming_json["data"];
                        StateUpdate::TimerLength(new_timer_length.as_str().unwrap().parse::<f32>().unwrap())
                    },

                    //Extra Toggles
                    "Toggle_Computer_Volume" => {StateUpdate::ToggleComputerSoundOn(string_to_bool(incoming_json["data"].as_str().unwrap()))},
                    "Toggle_Stream_Volume" => {StateUpdate::StreamSoundToggleOn(string_to_bool(incoming_json["data"].as_str().unwrap()))},
                    "Media_Pause_Play" => {StateUpdate::ComputerMediaDoPause},
                    "Timer_Text" => {StateUpdate::TimerText(incoming_json["data"].as_str().unwrap().to_string())}

                    "all" => {StateUpdate::UpdateClient},
                    
                    "Stream_Running" => {StateUpdate::StreamRunning(string_to_bool(incoming_json["data"].as_str().unwrap()))}

                    "Next_Slide" => {
                        if incoming_json["data"] == "hotkey" {StateUpdate::ChangeSlide(SlideChange::NextHotkey)}
                        else {StateUpdate::ChangeSlide(SlideChange::NextApp)}},
                    "Prev_Slide" => {
                        if incoming_json["data"] == "hotkey" {StateUpdate::ChangeSlide(SlideChange::PreviousHotkey)}
                        else {StateUpdate::ChangeSlide(SlideChange::PreviousApp)}}
                    //Unimplemented
                    _ => {panic!("trying to use a button type I don't know!: {}", value)}
                }
            },
            _ => {
                panic!("State Update Could Not Cast the json: {:?}", incoming_json.as_str());
            }
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        let (update_type, data) = match self {
            StateUpdate::StreamRunning(is_true) => {
                ("Stream_Running", is_true.to_string())},
            StateUpdate::StreamSoundToggleOn(is_true) => {
                ("Toggle_Stream_Volume", is_true.to_string())},
            StateUpdate::ToggleComputerSoundOn(is_true) => {
                ("Toggle_Computer_Volume", is_true.to_string())},
            StateUpdate::ChangeSceneOnChangeSlide(is_true) => {
               ("Change_With_Clicker", is_true.to_string())},
            StateUpdate::SceneIsAugmented(is_true) => {
                ("Scene_Is_Augmented",  is_true.to_string())},
            StateUpdate::TimerCanRun(is_true) => {
                ("Timer_Can_Run", is_true.to_string())},
            StateUpdate::TimerLength(length) => {
                ("Timer_Length", length.to_string())},
            StateUpdate::TimerText(text) => {
                ("Timer_Text", text.clone())},
            StateUpdate::SubScene(scene) => {
                ("SubScene", scene.to_string())},
            StateUpdate::Scene(scene) => {
                ("Scene", scene.to_string())},
            StateUpdate::ComputerMediaDoPause => {
                ("Toggle_Computer_Volume", "".to_string())},
            StateUpdate::ChangeSlide(value) => {
                match value {
                    SlideChange::NextApp => {("Next_Slide", "".to_string())},
                    SlideChange::NextHotkey => {("Next_Slide", "hotkey".to_string())},
                    SlideChange::PreviousApp => {("Prev_Slide", "".to_string())},
                     SlideChange::PreviousHotkey => {("Prev_Slide", "hotkey".to_string())},
                }
            },
            StateUpdate::PauseTimer(value) => {
                ("Pause_Timer", value.to_string())}
            StateUpdate::UpdateClient => {
                ("all", "".to_string())
            },
        };
    serde_json::json!({
        "type": "update",
        "update": update_type,
        "data": data,
        })
    }
}

fn string_to_bool(input: &str) -> bool{
    if input == "true" {
        true
    } else if input == "false" {
        false
    } else {
        panic!("string to bool doesn't recognize the input: {}", input);
    }
}