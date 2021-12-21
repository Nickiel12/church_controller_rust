use super::enums::{SubScenes, Scenes};
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
    pub camera_sub_scene: SubScenes,
    pub screen_sub_scene: SubScenes,
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
            
            current_scene: Scenes::Camera,
            camera_sub_scene: SubScenes::CameraDefault,
            screen_sub_scene: SubScenes::ScreenDefault,
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
            StateUpdate::StreamSoundToggleOn(new_val)     => {self.stream_is_muted = new_val;},
            StateUpdate::ToggleComputerSoundOn(new_val) => {self.computer_sound_is_on = new_val;},
            StateUpdate::ChangeSceneOnChangeSlide(new_val) => {self.change_scene_on_change_slide_hotkey = new_val;},
            StateUpdate::TimerCanRun(new_val)  => {self.timer_can_run = new_val;},
            StateUpdate::TimerLength(new_val)   => {self.timer_length  = new_val;},
            StateUpdate::TimerText(new_val)   => {self.timer_text    = new_val;},
            StateUpdate::Scene(new_val)       => {self.change_scene(&new_val)},
            StateUpdate::SubScene(new_val)      =>  {self.change_sub_scene(&new_val)},
            StateUpdate::SceneIsAugmented(new_val) => {
                self.scene_is_augmented = new_val;
                self.change_scene(&Scenes::Augmented)},
            StateUpdate::UpdateClient => todo!(),
            StateUpdate::ComputerMediaDoPause(do_pause) => todo!(),
            StateUpdate::ChangeSlide(value) => todo!(),
        }
    }

    pub fn change_scene(&mut self, scene: &Scenes) {
        match scene {
            Scenes::Augmented => {self.current_scene = *scene;}
            Scenes::Camera => {self.current_scene = *scene},
            Scenes::Screen => {self.current_scene = *scene}, 
        }
    }

    pub fn change_sub_scene(&mut self, scene: &SubScenes) {
        match scene {
            SubScenes::CameraDefault | SubScenes::CameraWithUpperRight | 
            SubScenes::CameraWithLargeUpperRight | SubScenes::CameraWithLowerRight 
            => {StreamState::set_camera_scene(self, scene)},
            SubScenes::ScreenDefault | SubScenes::ScreenWithUpperRight |
            SubScenes::ScreenWithLowerRight 
            => {StreamState::set_screen_scene(self, scene)},
        }
    }

    fn set_camera_scene(&mut self, scene: &SubScenes) {
        self.camera_sub_scene = scene.clone();
        self.current_scene = Scenes::Camera;
    }

    fn set_screen_scene(&mut self, scene: &SubScenes) {
        self.screen_sub_scene = scene.clone();
        self.current_scene = Scenes::Screen;
    }
}
