use std::time::{SystemTime};

use super::{stream_states::{state_update::StateUpdate, stream_states_class::StreamState, enums::{SlideChange, Scenes}}, external_interface::{Hotkeys}};

pub trait MessageHandler {                              //the first one goes to socket, the second propogates
    fn handle_update(&mut self, update: StateUpdate, hotkey_handler: &Hotkeys)
     -> (Option<StateUpdate>, Option<Vec<StateUpdate>>);
    fn get_states(&self) -> StreamState;
    fn tick(&mut self) -> (Option<StateUpdate>, Option<StateUpdate>);
}

impl MessageHandler for StreamState {
    fn handle_update(&mut self, update: StateUpdate, hotkey_handler: &Hotkeys)
     -> (Option<StateUpdate>, Option<Vec<StateUpdate>>) {

        if update != StateUpdate::UpdateClient{
            self.update(update.clone());
        }

        if self.debug_mode {
            return (None, None)
        }

        match update {
            StateUpdate::ChangeSlide(direction) => {
                if self.timer_can_run {
                    self.timer_finished = false;
                    self.timer_start = SystemTime::now();
                }
                match direction {
                    SlideChange::Next => {
                        hotkey_handler.next_slide();
                    },
                    SlideChange::Previous => {
                        hotkey_handler.prev_slide();
                    }
                }
                if self.change_scene_on_change_slide_hotkey {
                    let mut instructions = Vec::new();
                    instructions.push(StateUpdate::Scene(Scenes::Screen));

                    return (None, Some(instructions))
                } else {return (None, None)}
            }
            StateUpdate::ChangeSceneOnChangeSlide(value) => {self.change_scene_on_change_slide_hotkey = value; return (Some(update), None)},
            StateUpdate::SceneIsAugmented(value) => {
                if value {
                    let mut instructions = Vec::new();
                    instructions.push(StateUpdate::ChangeSceneOnChangeSlide(false));
                    instructions.push(StateUpdate::Scene(Scenes::Augmented));
                    instructions.push(StateUpdate::TimerCanRun(false));
                    return (Some(update), Some(instructions))
                } else {
                    let mut instructions = Vec::new();
                    instructions.push(StateUpdate::Scene(Scenes::Camera));
                    instructions.push(StateUpdate::ChangeSceneOnChangeSlide(true));
                    instructions.push(StateUpdate::TimerCanRun(true));
                    return (Some(update), Some(instructions));
                }
            },
            StateUpdate::TimerCanRun(value) => {self.timer_can_run = value; return (Some(update), None)},
            StateUpdate::TimerLength(value) => {self.timer_length = value; return (Some(update), None)},
            StateUpdate::TimerText(value) => {self.timer_text = value.clone(); return (Some(StateUpdate::TimerText(value)), None)},
            StateUpdate::SubScene(value) => {
                if value.get_type() == Scenes::Camera {
                    self.camera_sub_scene = value;
                    if self.current_scene == Scenes::Camera {
                        hotkey_handler.change_scene(Scenes::Camera, Some(self.camera_sub_scene));
                    }
                    return (Some(update), None)
                } else if value.get_type() == Scenes::Screen {
                    self.screen_sub_scene = value;
                    if self.current_scene == Scenes::Screen {
                        hotkey_handler.change_scene(Scenes::Screen, Some(self.screen_sub_scene));
                    }
                    return (Some(update), None)
                }
            },
            StateUpdate::Scene(value) => {
                hotkey_handler.change_scene(value, None);
                self.current_scene = value;
                return (Some(update), None);
            },
            StateUpdate::StreamSoundToggleOn(value) => {hotkey_handler.toggle_stream_sound(value); return (Some(update), None)},
            StateUpdate::ToggleComputerSoundOn(value) => {hotkey_handler.toggle_computer_sound(value); return (Some(update), None)},
            StateUpdate::ComputerMediaDoPause => {hotkey_handler.toggle_media_play_pause(); return (Some(update), None)},
            StateUpdate::UpdateClient => {},
            StateUpdate::StreamRunning(_) => {},
            //_ => {}
        }
        (None, None)
    }

    fn tick(&mut self) -> (Option<StateUpdate>, Option<StateUpdate>) {
        if self.timer_finished == false {
            let change = self.timer_start.elapsed();
            match change {
                Err(_) => {(None, None)},
                Ok(change) => {
                    if change.as_secs_f32() >= self.timer_length {
                        self.timer_finished = true;
                        (Some(StateUpdate::TimerText(String::from("0.0"))),
                        Some(StateUpdate::Scene(Scenes::Camera)))
                    } else {
                        (Some(StateUpdate::TimerText(
                            format!("{:.1}", self.timer_length - ((change.as_secs_f32() * 10.0).round() / 10.0))
                        )), None)
                    }
                }
            }
        } else {
            (None, None)
        }
    }

    fn get_states(&self) -> StreamState{
        self.clone()
    }

}

#[test]
fn test_tick_1() {
    let mut state = StreamState::new();
    state.timer_finished = false;
    state.tick();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    let update = state.tick();
    state.update(update.0.unwrap());
    assert_eq!(state.timer_text, "14.0");
}

#[test]
fn test_tick_one_half() {
    let mut state = StreamState::new();
    state.timer_finished = false;
    state.tick();
    std::thread::sleep(std::time::Duration::from_millis(500));
    let update = state.tick();
    state.update(update.0.unwrap());
    assert_eq!(state.timer_text, "14.5");
}

#[test]
#[ignore]
fn test_tick_10() {
    let mut state = StreamState::new();
    state.timer_finished = false;
    state.tick();
    std::thread::sleep(std::time::Duration::from_millis(10000));
    let update = state.tick();
    state.update(update.0.unwrap());
    assert_eq!(state.timer_text, "5.0");
}