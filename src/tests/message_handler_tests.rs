use crate::modules::{message_handler::{MessageHandler}, stream_states::{state_update::StateUpdate, stream_states_class::StreamState}};


#[test]
fn does_stream_state_implement_message_handler() {
    let mut state = StreamState::new();
    state.debug_mode = true;
    state.handle_update(StateUpdate::ToggleComputerSoundOn(false));
    assert_eq!(state.computer_sound_is_on, false);
}
