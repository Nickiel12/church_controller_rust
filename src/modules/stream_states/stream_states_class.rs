use super::enums::{self, StateUpdate};

#[derive(Debug, Clone)]
pub struct StreamState {
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

impl Default for StreamState {
    fn default() -> Self {
        StreamState {  
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

impl StreamState {
    pub fn new() -> Self {
        StreamState{..Default::default()}
    }

    pub fn update(&mut self, update: StateUpdate) {
        match update {
            StateUpdate::StreamRunning(new_val)     => {self.stream_running  = new_val;},
            StateUpdate::StreamIsMuted(new_val)     => {self.stream_is_muted = new_val;},
            StateUpdate::ComputerSoundIsOn(new_val) => {self.computer_sound_is_on = new_val;},
            StateUpdate::ChangeSceneOnChangeSlideHotkey(new_val) => {self.change_scene_on_change_slide_hotkey = new_val;},
            StateUpdate::TimerCanRun(new_val)  => {self.timer_can_run = new_val;},
            StateUpdate::TimerLength(new_val)   => {self.timer_length  = new_val;},
            StateUpdate::TimerText(new_val)  =>  {self.timer_text    = new_val;},
            StateUpdate::Scene(new_val)      =>  {self.change_scene(&new_val)},
            StateUpdate::SceneIsAugmented(new_val) => {
                self.scene_is_augmented = new_val;
                self.change_scene(&enums::Scenes::Augmented)},
        }
    }

    pub fn change_scene(&mut self, scene: &enums::Scenes) {
        match scene {
            enums::Scenes::CameraDefault | enums::Scenes::CameraWithUpperRight | 
            enums::Scenes::CameraWithLargeUpperRight | enums::Scenes::CameraWithLowerRight 
            => {StreamState::set_camera_scene(self, scene)},
            enums::Scenes::ScreenDefault | enums::Scenes::ScreenWithUpperRight |
            enums::Scenes::ScreenWithLowerRight 
            => {StreamState::set_screen_scene(self, scene)},
            enums::Scenes::Augmented => {self.current_scene = *scene;}   
        }
    }

    fn set_camera_scene(&mut self, scene: &enums::Scenes) {
        self.camera_sub_scene = scene.clone();
        self.current_scene = scene.clone();
    }

    fn set_screen_scene(&mut self, scene: &enums::Scenes) {
        self.screen_sub_scene = scene.clone();
        self.current_scene = scene.clone();
    }
}
