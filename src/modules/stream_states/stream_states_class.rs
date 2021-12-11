use super::enums::{self};

pub enum StateUpdate {
    StreamRunning(bool),
    StreamIsMuted(bool),
    ComputerSoundIsOn(bool),
    ChangeSceneOnChangeSlideHotkey(bool),
    SceneIsAugmented(bool),
    TimerCanRun(bool),
    TimerLength(f32),
    TimerText(String),
    Scene(enums::Scenes),
}

#[derive(Debug)]
pub struct StreamStates {
    pub stream_running: bool,
    pub stream_is_muted: bool,
    pub computer_sound_is_on: bool,

    pub change_scene_on_change_slide_hotkey: bool,
    pub scene_is_augmented: bool,

    pub timer_can_run: bool,
    pub timer_length: f32,
    pub timer_text: String,

    current_scene: enums::Scenes,
    camera_sub_scene: enums::Scenes,
    screen_sub_scene: enums::Scenes,
}

impl Default for StreamStates {
    fn default() -> Self {
        StreamStates {  
            stream_running: false,
            stream_is_muted: false,
            computer_sound_is_on: true,

            change_scene_on_change_slide_hotkey: true,
            scene_is_augmented: false,

            timer_can_run: true,
            timer_length: 15.0,
            timer_text: String::from("0.0"),
            
            current_scene: enums::Scenes::CameraDefault,
            camera_sub_scene: enums::Scenes::CameraDefault,
            screen_sub_scene: enums::Scenes::ScreenDefault,
        }
    }
}

impl StreamStates {
    pub fn new() -> Self {
        StreamStates{..Default::default()}
    }

    pub fn update(mut self, updateMessage: StateUpdate) -> Self {
        match updateMessage {
            StateUpdate::StreamRunning(new_val) => {self.stream_running = new_val; self},
            StateUpdate::StreamIsMuted(new_val) => {self.stream_is_muted = new_val; self},
            StateUpdate::ComputerSoundIsOn(new_val) => {self.computer_sound_is_on = new_val; self},
            StateUpdate::ChangeSceneOnChangeSlideHotkey(new_val) => {self.change_scene_on_change_slide_hotkey = new_val; self},
            StateUpdate::TimerCanRun(new_val) => {self.timer_can_run = new_val; self},
            StateUpdate::TimerLength(new_val) => {self.timer_length = new_val; self},
            StateUpdate::TimerText(new_val) => {self.timer_text = new_val; self},
            StateUpdate::Scene(new_val) => {self.change_scene(&new_val)},
            StateUpdate::SceneIsAugmented(new_val) => {
                self.scene_is_augmented = new_val;
                self.change_scene(&enums::Scenes::Augmented)},
        }
    }

    pub fn get_current_scene(&self) -> enums::Scenes {
        self.current_scene
    }

    pub fn get_current_camera_sub_scene(&self) -> enums::Scenes {
        self.camera_sub_scene
    }

    pub fn get_current_screen_sub_scene(&self) -> enums::Scenes {
        self.screen_sub_scene
    }

    pub fn change_scene(mut self, scene: &enums::Scenes) -> Self {
        match scene {
            enums::Scenes::CameraDefault | enums::Scenes::CameraWithUpperRight | 
            enums::Scenes::CameraWithLargeUpperRight | enums::Scenes::CameraWithLowerRight 
            => {StreamStates::set_camera_scene(self, scene)},
            enums::Scenes::ScreenDefault | enums::Scenes::ScreenWithUpperRight |
            enums::Scenes::ScreenWithLowerRight 
            => {StreamStates::set_screen_scene(self, scene)},
            enums::Scenes::Augmented => {self.current_scene = *scene; self}   
        }
    }

    fn set_camera_scene(mut self, scene: &enums::Scenes) -> Self{
        self.camera_sub_scene = scene.clone();
        self.current_scene = scene.clone();
        self
    }

    fn set_screen_scene(mut self, scene: &enums::Scenes) -> Self {
        self.screen_sub_scene = scene.clone();
        self.current_scene = scene.clone();
        self
    }
}
