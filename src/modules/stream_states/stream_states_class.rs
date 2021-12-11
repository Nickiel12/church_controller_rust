use super::enums::{self};


pub struct StreamStates {
    pub stream_running: bool,
    pub stream_is_muted: bool,
    pub computer_sound_is_on: bool,

    pub change_scene_on_change_slide_hotkey: bool,
    pub scene_is_augmented: bool,

    pub timer_can_run: bool,
    pub timer_length: f32,
    pub timer_text: String,

    pub current_scene: enums::Scenes,
    pub camera_sub_scene: enums::Scenes,
    pub screen_sub_scene: enums::Scenes,
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

    pub fn change_scene(self, scene: &enums::Scenes) -> Self {
        match scene {
            enums::Scenes::CameraDefault | enums::Scenes::CameraWithUpperRight | 
            enums::Scenes::CameraWithLargeUpperRight | enums::Scenes::CameraWithLowerRight 
            => {StreamStates::set_camera_scene(self, scene)},
            enums::Scenes::ScreenDefault | enums::Scenes::ScreenWithUpperRight |
            enums::Scenes::ScreenWithLowerRight 
            => {StreamStates::set_screen_scene(self, scene)},
            
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
