use std::{sync::mpsc::{self, Receiver}, thread::{self, JoinHandle}};

use crate::modules::{message_handler::{MessageHandler, StateMessage}, stream_states::{state_update::StateUpdate, enums::{Scenes}, stream_states_class::StreamState}};


#[test]
fn does_stream_state_implement_message_handler() {
    let mut state = StreamState::new();
    state.handle_update(StateUpdate::ComputerSoundIsOn(false));
    assert_eq!(state.computer_sound_is_on, false);
}

#[test]
fn message_hander_can_handle_basic_message() {
    let (tx, rx) = mpsc::channel();

    fn listen(mut handler: StreamState, rx: Receiver<StateMessage>) -> JoinHandle<()>{
        thread::spawn(move || {
            for message in rx {
                match message {
                    StateMessage::StateUpdate(update) => {handler.handle_update(update)},
                    StateMessage::GetStates => {panic!("Why is getstates being called in this test?");},
                    StateMessage::CloseListener => {break;},
                };
            }
            assert_eq!(handler.computer_sound_is_on, false);
        })
    }

    let handler = StreamState::new();
    
    let join_handle = listen(handler, rx);
    tx.send(StateMessage::StateUpdate(StateUpdate::ComputerSoundIsOn(false))).unwrap();
    thread::sleep(std::time::Duration::from_millis(1000));
    tx.send(StateMessage::CloseListener).unwrap();
    join_handle.join().unwrap();
}

#[test]
fn message_handler_can_handle_multiple_messages() {
    let (tx, rx) = mpsc::channel();
    
    
    fn listen(mut handler: StreamState, rx: Receiver<StateMessage>) -> JoinHandle<()>{
        thread::spawn(move || {
            for message in rx {
                match message {
                    StateMessage::StateUpdate(update) => {handler.handle_update(update)},
                    StateMessage::GetStates => {panic!("Why is getstates being called in this test?");},
                    StateMessage::CloseListener => {break;},
                };
            }
            assert_eq!(handler.computer_sound_is_on, false);
            assert_eq!(handler.scene_is_augmented, true);
            assert_eq!(handler.scene_is_augmented, true);
            assert_eq!(handler.current_scene, Scenes::Augmented);
            assert_eq!(handler.timer_can_run, false);
        })
    }
    

    let handler = StreamState::new();
    
    let join_handle = listen(handler, rx);
    tx.send(StateMessage::StateUpdate(StateUpdate::ComputerSoundIsOn(false))).unwrap();
    tx.send(StateMessage::StateUpdate(StateUpdate::SceneIsAugmented(true))).unwrap();
    tx.send(StateMessage::StateUpdate(StateUpdate::TimerCanRun(false))).unwrap();
    thread::sleep(std::time::Duration::from_millis(1000));
    tx.send(StateMessage::CloseListener).unwrap();
    join_handle.join().unwrap();
}   
