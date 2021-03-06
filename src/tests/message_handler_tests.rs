use crate::modules::{
    external_interface::Hotkeys,
    message_handler::MessageHandler,
    stream_states::{state_update::StateUpdate, stream_state::StreamState},
};

#[test]
fn does_stream_state_implement_message_handler() {
    let hotkeys = Hotkeys {
        hotkeys: serde_json::Value::Null,
    };
    let mut state = StreamState::new();
    state.debug_mode = true;
    state.handle_update(StateUpdate::ToggleComputerSoundOn(false), &hotkeys);
    assert_eq!(state.computer_sound_is_on, false);
}
