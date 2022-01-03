use std::{time::Duration, thread, io::Read};
use crossbeam_channel::unbounded;

use modules::{socket_handler::Socket, stream_states::stream_states_class::StreamState, message_handler::{MessageHandler}, external_interface::{Hotkeys, OPTIONS_PATH}};
use workctl::sync_flag;

use crate::modules::stream_states::state_update::StateUpdate;

#[cfg(test)]
mod tests;
mod modules;


#[cfg(target_os = "windows")]
const SERVER_ADDRESS: &str = "10.0.0.209:5000";

#[cfg(target_os = "linux")]
const SERVER_ADDRESS: &str = "10.0.0.168:5000";

fn main() {
    let settings_json = load_json();
    let hotkeys = Hotkeys::new(settings_json);
    
    let (from_socket_tx, from_socket_rx) = unbounded::<String>();
    let hotkey_channel_tx = from_socket_tx.clone();
    
    let mut socket = Socket::handle_connections(Socket::make_listener(SERVER_ADDRESS), from_socket_tx);
    
    let (hotkey_close_flag_tx, hotkey_close_flag_rx) = sync_flag::new_syncflag(true);
    let control_c_called_flag_rx = setup_control_c(hotkey_close_flag_tx);
    
    let hotkey_handle = thread::spawn(move || {
        modules::external_interface::create_keyboard_hooks(hotkey_channel_tx, hotkey_close_flag_rx);
    });
    
    let mut state = StreamState::new();
    //until control_c is caught, check the queue of incoming
    //requests from the socket handler.
    while !control_c_called_flag_rx.get() {
        match from_socket_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(message) => {
                println!("main recieved: {}", message);
                let json = serde_json::from_str(&message).unwrap();
                let update = StateUpdate::json_to_state_update(json);
                if update == StateUpdate::UpdateClient {
                    update_all(&state, &socket);
                }
                let updates = state.handle_update(update, &hotkeys);
                if updates.0.is_some() {
                    socket.send(updates.0.unwrap().to_json().to_string());
                }
                if updates.1.is_some() {
                    handle_instructions(updates.1.unwrap(), &mut state, &socket, &hotkeys);
                }
            },
            Err(_) => {},
        }
        let tick_update = state.tick();
        handle_instructions(tick_update, &mut state, &socket, &hotkeys);
    }
    
    socket.close();
    hotkey_handle.join().unwrap();
}

fn handle_instructions(mut instructions: Vec<StateUpdate>, state: &mut StreamState, socket: &Socket, hotkeys: &Hotkeys) {
    for i in instructions.iter_mut() {
        let updates = state.handle_update(i.to_owned(), &hotkeys);
        if updates.0.is_some() {
            socket.send(updates.0.unwrap().to_json().to_string());
        }
        if updates.1.is_some() {
            handle_instructions(updates.1.unwrap(), state, socket, hotkeys);
        }
    }
}

fn setup_control_c(mut hotkey_close_flag_tx: sync_flag::SyncFlagTx) -> sync_flag::SyncFlagRx{
    let (mut control_c_flag_tx, control_c_called_flag_rx) = sync_flag::new_syncflag(false);
    ctrlc::set_handler(move || {
        println!("ctrl c caught");
        control_c_flag_tx.set(true);
        hotkey_close_flag_tx.set(false);
    }).expect("control C handler failed!");
    control_c_called_flag_rx
}

fn load_json() -> serde_json::Value {
        let mut settings_file = std::fs::File::open(OPTIONS_PATH).unwrap();
        let mut settings_str = String::new();
        settings_file.read_to_string(&mut settings_str).unwrap();
        drop(settings_file);
        serde_json::from_str(settings_str.as_str()).unwrap()
}

fn update_all(state: &StreamState, socket: &Socket) {
    println!("updating all");
    socket.send(StateUpdate::StreamRunning(state.stream_running).to_json().to_string());
    socket.send(StateUpdate::StreamSoundToggleOn(state.stream_is_muted).to_json().to_string());
    socket.send(StateUpdate::ToggleComputerSoundOn(state.computer_sound_is_on).to_json().to_string());
    socket.send(StateUpdate::ChangeSceneOnChangeSlide(state.change_scene_on_change_slide_hotkey).to_json().to_string());
    socket.send(StateUpdate::SceneIsAugmented(state.scene_is_augmented).to_json().to_string());
    socket.send(StateUpdate::TimerCanRun(state.timer_can_run).to_json().to_string());
    socket.send(StateUpdate::TimerLength(state.timer_length).to_json().to_string());
    socket.send(StateUpdate::TimerText(state.timer_text.clone()).to_json().to_string());
    socket.send(StateUpdate::SubScene(state.camera_sub_scene).to_json().to_string());
    socket.send(StateUpdate::SubScene(state.screen_sub_scene).to_json().to_string());
    socket.send(StateUpdate::Scene(state.current_scene).to_json().to_string());
}

