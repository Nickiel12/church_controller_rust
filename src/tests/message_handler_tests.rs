use std::{sync::mpsc::{self, Receiver}, thread::{self, JoinHandle}};

use crate::modules::{message_handler::{MessageHandler, StateMessage}, stream_states::{enums::{StateUpdate, Scenes}}};




#[test]
fn create_message_handler() {
    MessageHandler::new();
}

#[test]
fn message_hander_can_handle_basic_message() {
    let (tx, rx) = mpsc::channel();

    fn listen(mut handler: MessageHandler, rx: Receiver<StateMessage>) -> JoinHandle<()>{
        thread::spawn(move || {
            for message in rx {
                match message {
                    StateMessage::StateUpdateContainer(update) => {handler.handle_update(update)},
                    StateMessage::CloseListener => {break;},
                };
            }
            assert_eq!(handler.state.computer_sound_is_on, false);
        })
    }

    let handler = MessageHandler::new();
    
    let join_handle = listen(handler, rx);
    tx.send(StateMessage::StateUpdateContainer(StateUpdate::ComputerSoundIsOn(false))).unwrap();
    thread::sleep(std::time::Duration::from_millis(1000));
    tx.send(StateMessage::CloseListener).unwrap();
    join_handle.join().unwrap();
}

#[test]
fn message_handler_can_handle_multiple_messages() {
    let (tx, rx) = mpsc::channel();
    
    
    fn listen(mut handler: MessageHandler, rx: Receiver<StateMessage>) -> JoinHandle<()>{
        thread::spawn(move || {
            for message in rx {
                match message {
                    StateMessage::StateUpdateContainer(update) => {handler.handle_update(update)},
                    StateMessage::CloseListener => {break;},
                };
            }
            assert_eq!(handler.state.computer_sound_is_on, false);
            assert_eq!(handler.state.scene_is_augmented, true);
            assert_eq!(handler.state.scene_is_augmented, true);
            assert_eq!(handler.state.get_current_scene(), Scenes::Augmented);
            assert_eq!(handler.state.timer_can_run, false);
        })
    }
    

    let handler = MessageHandler::new();
    
    let join_handle = listen(handler, rx);
    tx.send(StateMessage::StateUpdateContainer(StateUpdate::ComputerSoundIsOn(false))).unwrap();
    tx.send(StateMessage::StateUpdateContainer(StateUpdate::SceneIsAugmented(true))).unwrap();
    tx.send(StateMessage::StateUpdateContainer(StateUpdate::TimerCanRun(false))).unwrap();
    thread::sleep(std::time::Duration::from_millis(1000));
    tx.send(StateMessage::CloseListener).unwrap();
    join_handle.join().unwrap();
}   
