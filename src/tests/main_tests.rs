use workctl::sync_flag;

use crate::modules::{stream_states::{ stream_state::StreamState, scenes::Scenes, state_update::StateUpdate}};


#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn can_make_ctrl_c_handler() {
    let (control_c_flag_tx, _control_c_called_flag_rx) = sync_flag::new_syncflag(false);
    crate::setup_control_c(control_c_flag_tx);
    drop(_control_c_called_flag_rx);
}


#[test]
fn test_updating_state_from_state_update() {
    let mut state = StreamState::new();
    state.debug_mode = true;
    let update = StateUpdate::Scene(Scenes::Augmented);

    //Update handled extensivly in stream_state_tests
    state.update(update);
    assert_eq!(state.current_scene, Scenes::Augmented);
}
