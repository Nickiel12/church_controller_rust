use std::process::Command;
use super::stream_states::{state_update::StateUpdate, enums::{SlideChange, SubScenes, Scenes}};

const AHK_FILES_FOLDER: &str = ".\\ahk_files\\";
pub const OPTIONS_PATH: &str = ".\\options.json";

    /*
    const AHK_FILES_FOLDER: &str = "./src/ahk_files/";
    pub const OPTIONS_PATH: &str = "./options.json";
    */

pub fn create_keyboard_hooks(channel_tx: crossbeam_channel::Sender<String>, close_flag: workctl::sync_flag::SyncFlagRx) {
    
    let tx_1 = channel_tx.clone();
    inputbot::KeybdKey::PageUpKey.bind(move || {
        tx_1.send(StateUpdate::ChangeSlide(SlideChange::Next).to_json().to_string()).unwrap();
    });

    let tx_2 = channel_tx.clone();
    inputbot::KeybdKey::PageDownKey.bind(move || {
        tx_2.send(StateUpdate::ChangeSlide(SlideChange::Previous).to_json().to_string()).unwrap();
    });
    
    inputbot::handle_input_events(close_flag);
}

pub struct Hotkeys {
    pub hotkeys: serde_json::Value,
}

impl Hotkeys {
    pub fn get_hotkey_from_scene(&self, scene: SubScenes) -> &str {
        match scene {
            SubScenes::CameraDefault => {self.hotkeys["hotkeys"]["obs"]["camera_scene_hotkey"].as_str().unwrap()},
            SubScenes::CameraWithUpperRight => {self.hotkeys["hotkeys"]["obs"]["Camera_Top_Right"].as_str().unwrap()},
            SubScenes::CameraWithLargeUpperRight => {self.hotkeys["hotkeys"]["obs"]["Camera_Large_Top_Right"].as_str().unwrap()},
            SubScenes::CameraWithLowerRight => {self.hotkeys["hotkeys"]["obs"]["Camera_Bottom_Right"].as_str().unwrap()},
            SubScenes::ScreenDefault => {self.hotkeys["hotkeys"]["obs"]["screen_scene_hotkey"].as_str().unwrap()},
            SubScenes::ScreenWithUpperRight => {self.hotkeys["hotkeys"]["obs"]["Screen_Top_Right"].as_str().unwrap()},
            SubScenes::ScreenWithLowerRight => {self.hotkeys["hotkeys"]["obs"]["Screen_Bottom_Right"].as_str().unwrap()},
        }
    }
    pub fn send_obs(&self, hotkey: &str) {
        if cfg!(target_family = "windows") {
            Command::new(String::from(AHK_FILES_FOLDER) + "send_obs_back_to_propre.exe")
                .args([self.hotkeys["windows"]["propresenter_re"].as_str().unwrap(),
                       self.hotkeys["windows"]["obs_re"].as_str().unwrap(),
                       hotkey])
                .spawn()
                .expect("next_slide process call failed");
        } else {
            println!("pretend linux is sending obs send: {}", hotkey)
        };
    }

    pub fn next_slide(&self) {
        if cfg!(target_family = "windows") {
            Command::new(String::from(AHK_FILES_FOLDER) + "propre_send.exe")
                .args([self.hotkeys["windows"]["propresenter_re"].as_str().unwrap(), 
                       self.hotkeys["general"]["clicker_forward"].as_str().unwrap()])
                .spawn()
                .expect("next_slide process call failed");
        } else {
            println!("pretend linux is sending prosenter next: {}", self.hotkeys["general"]["clicker_forward"].as_str().unwrap())
        };
    }
    
    pub fn prev_slide(&self) {
        if cfg!(target_family = "windows") {
            Command::new(String::from(AHK_FILES_FOLDER) + "propre_send.exe")
                .args([self.hotkeys["windows"]["propresenter_re"].as_str().unwrap(), 
                       self.hotkeys["general"]["clicker_backward"].as_str().unwrap()])
                .spawn()
                .expect("next_slide process call failed");
        } else {
            println!("pretend linux is sending prosenter next: {}", self.hotkeys["general"]["clicker_backward"].as_str().unwrap())
        };
    }

    pub fn change_scene(&self, scene: Scenes, sub_scene: Option<SubScenes>) {
        let hotkey: &str;
        if scene == Scenes::Augmented {
            hotkey = self.hotkeys["hotkeys"]["obs"]["camera_scene_augmented"].as_str().unwrap()
        } else {
            hotkey = self.get_hotkey_from_scene(sub_scene.unwrap())
        };
        self.send_obs(hotkey);
    }
    
    pub fn toggle_stream_sound(&self, turn_on: bool) {
        let hotkey: &str;
        if turn_on {
            hotkey = self.hotkeys["hotkeys"]["obs"]["unmute_stream"].as_str().unwrap();
        } else {
            hotkey = self.hotkeys["hotkeys"]["obs"]["mute_stream"].as_str().unwrap();
        }
        self.send_obs(hotkey);
    }
    
    pub fn toggle_computer_sound(&self, value: bool) {
        let direction: u8 = if value {1} else {0};
        let time_delay = self.hotkeys["general"]["music_fade_time"].as_i64().unwrap();
        if cfg!(target_family = "windows") {
            Command::new(String::from(AHK_FILES_FOLDER) + "music_toggle.exe")
                .arg(direction.to_string()) 
                .arg(time_delay.to_string())
                .spawn()
                .expect("next_slide process call failed");
        } else {
            println!("pretend linux is sending prosenter next: {}", self.hotkeys["general"]["clicker_backward"].as_str().unwrap())
        };
    }
    
    pub fn toggle_media_play_pause(&self) {
        if cfg!(target_family = "windows") {
            Command::new(String::from(AHK_FILES_FOLDER) + "music_toggle.exe")
                .arg(self.hotkeys["windows"]["propresenter_re"].as_str().unwrap()) 
                .spawn()
                .expect("next_slide process call failed");
        } else {
            println!("pretend linux is sending prosenter next: {}", self.hotkeys["general"]["clicker_backward"].as_str().unwrap())
        };
    }    
}


#[test]
fn hotkeys() {
    use std::io::Read;
    let settings_json: serde_json::Value;
    {
        let mut settings_file = std::fs::File::open(OPTIONS_PATH).unwrap();
        let mut settings_str = String::new();
        settings_file.read_to_string(&mut settings_str).unwrap();
        settings_json = serde_json::from_str(settings_str.as_str()).unwrap();
        drop(settings_file);
    }
    let hk = Hotkeys {
        hotkeys: settings_json,
    };
    hk.change_scene(Scenes::Augmented, Some(SubScenes::CameraDefault));
    hk.next_slide();
    hk.prev_slide();
    hk.send_obs("a hotkey");
    hk.toggle_computer_sound(true);
    hk.toggle_stream_sound(true);
    hk.toggle_media_play_pause();
}