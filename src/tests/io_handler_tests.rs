use std::thread;

use crate::modules::{io_handler::{IOHandler}, message_handler::{MessageHandler, StateMessage}, stream_states::{stream_states_class::StreamState, enums::StateUpdate}};


#[test]
fn test_make_socket() {
    let state = StreamState::new();
    let mut io_handler = IOHandler::new();

    io_handler.start_socket_listener(state, "no-one cares");

    let tx = io_handler.message_thread_tx.clone().unwrap();
    tx.send(StateMessage::StateUpdate(StateUpdate::SceneIsAugmented(true))).unwrap();
    tx.send(StateMessage::StateUpdate(StateUpdate::StreamIsMuted(true))).unwrap();
    thread::sleep(std::time::Duration::from_millis(1000));

    let final_state = io_handler.close();
    assert_eq!(final_state.scene_is_augmented, true);
    assert_eq!(final_state.stream_is_muted, true);
}