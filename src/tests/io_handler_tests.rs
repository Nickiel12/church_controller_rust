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

#[test]
fn send_on_multiple_threads() {
    let state = StreamState::new();
    let mut io_handler = IOHandler::new();

    io_handler.start_socket_listener(state, "no-one cares");

    let tx1 = io_handler.message_thread_tx.clone().unwrap();
    let tx2 = io_handler.message_thread_tx.clone().unwrap();
    let tx3 = io_handler.message_thread_tx.clone().unwrap();

    thread::spawn( move || {
        tx1.send(StateMessage::StateUpdate(StateUpdate::TimerCanRun(false))).unwrap();
    });
    thread::spawn(move || {
        tx2.send(StateMessage::StateUpdate(StateUpdate::ChangeSceneOnChangeSlideHotkey(false))).unwrap();
    });
    thread::spawn(move || {
        tx3.send(StateMessage::StateUpdate(StateUpdate::StreamIsMuted(false))).unwrap();
        tx3.send(StateMessage::StateUpdate(StateUpdate::StreamIsMuted(true))).unwrap();
    });

    thread::sleep(std::time::Duration::from_millis(1000));

    let final_state = io_handler.close();

    assert_eq!(final_state.timer_can_run, false);
    assert_eq!(final_state.change_scene_on_change_slide_hotkey, false);
    assert_eq!(final_state.stream_is_muted, true);
}