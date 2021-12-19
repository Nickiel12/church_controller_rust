use super::enums::Scenes;
use super::state_update::StateUpdate;

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

    pub current_scene: Scenes,
    pub camera_sub_scene: Scenes,
    pub screen_sub_scene: Scenes,
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
            
            current_scene: Scenes::CameraDefault,
            camera_sub_scene: Scenes::CameraDefault,
            screen_sub_scene: Scenes::ScreenDefault,
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
                self.change_scene(&Scenes::Augmented)},
        }
    }

    pub fn change_scene(&mut self, scene: &Scenes) {
        match scene {
            Scenes::CameraDefault | Scenes::CameraWithUpperRight | 
            Scenes::CameraWithLargeUpperRight | Scenes::CameraWithLowerRight 
            => {StreamState::set_camera_scene(self, scene)},
            Scenes::ScreenDefault | Scenes::ScreenWithUpperRight |
            Scenes::ScreenWithLowerRight 
            => {StreamState::set_screen_scene(self, scene)},
            Scenes::Augmented => {self.current_scene = *scene;}   
        }
    }

    fn set_camera_scene(&mut self, scene: &Scenes) {
        self.camera_sub_scene = scene.clone();
        self.current_scene = scene.clone();
    }

    fn set_screen_scene(&mut self, scene: &Scenes) {
        self.screen_sub_scene = scene.clone();
        self.current_scene = scene.clone();
    }
}
