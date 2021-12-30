use super::stream_states::{state_update::StateUpdate, enums::{SlideChange, SubScenes, Scenes}};

const AHK_FILES_FOLDER: &str = "./src/ahk_files";
pub const OPTIONS_PATH: &str = "./options.json";

pub fn create_keyboard_hooks(channel_tx: crossbeam_channel::Sender<String>) {
    
    let tx_1 = channel_tx.clone();
    inputbot::KeybdKey::PageUpKey.bind(move || {
        tx_1.send(StateUpdate::ChangeSlide(SlideChange::Next).to_json().to_string()).unwrap();
    });

    let tx_2 = channel_tx.clone();
    inputbot::KeybdKey::PageDownKey.bind(move || {
        tx_2.send(StateUpdate::ChangeSlide(SlideChange::Previous).to_json().to_string()).unwrap();
    });
    
    inputbot::handle_input_events();
}

pub struct Hotkeys {
    pub hotkeys: serde_json::Value,
}

impl Hotkeys {
    pub fn next_slide(&self, ) {
        todo!()
    }
    
    pub fn prev_slide(&self, ) {
        todo!()
    }
    
    pub fn toggle_stream_sound(&self, value: bool) {
        todo!()
    }
    
    pub fn toggle_computer_sound(&self, value: bool) {
        todo!()
    }
    
    pub fn toggle_media_play_pause(&self, value: bool) {
        todo!()
    }
    
    pub fn change_scene(&self, scene: Scenes, sub_scene: Option<SubScenes>) {
        todo!()
    }
}
