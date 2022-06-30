use std::time::{SystemTime};

use super::{stream_states::{state_update::StateUpdate, stream_state::StreamState, scenes::{SlideChange, Scenes}}, external_interface::{Hotkeys}};

pub trait MessageHandler {                              //the first one goes to socket, the second propogates
    fn handle_update(&mut self, update: StateUpdate, hotkey_handler: &Hotkeys)
     -> (Option<StateUpdate>, Option<Vec<StateUpdate>>);
    fn get_states(&self) -> StreamState;
    fn pause_timer(&mut self, do_pause: bool) -> (Option<StateUpdate>, Option<Vec<StateUpdate>>);
    fn tick(&mut self) -> Vec<StateUpdate>;
}

impl MessageHandler for StreamState {
    fn handle_update(&mut self, update: StateUpdate, hotkey_handler: &Hotkeys)
     -> (Option<StateUpdate>, Option<Vec<StateUpdate>>) {

        if update != StateUpdate::UpdateClient && update != StateUpdate::ChangeSlide(SlideChange::NextApp) &&
            update != StateUpdate::ChangeSlide(SlideChange::PreviousApp) && update != StateUpdate::ChangeSlide(SlideChange::PreviousHotkey)
            && update != StateUpdate::ChangeSlide(SlideChange::NextHotkey) {
            self.update(update.clone());
        }

        if self.debug_mode {
            return (None, None)
        }

        match update {
            StateUpdate::ChangeSlide(direction) => {
                
                match direction {
                    SlideChange::NextHotkey => {
                        hotkey_handler.next_slide(true);
                    },
                    SlideChange::NextApp => {
                        hotkey_handler.next_slide(false);
                    },
                    SlideChange::PreviousHotkey => {
                        hotkey_handler.prev_slide(true);
                    },
                    SlideChange::PreviousApp => {
                        hotkey_handler.prev_slide(false);
                    },
                }
                
                if self.change_scene_on_slide_hotkey {
                    if self.timer_can_run {
                        self.timer_finished = false;
                        self.timer_start = SystemTime::now();
                    }
                    let mut instructions = Vec::new();
                    instructions.push(StateUpdate::Scene(Scenes::Screen));

                    return (None, Some(instructions))
                } else {return (None, None)}
            }
            StateUpdate::ChangeSceneOnChangeSlide(value) => {self.change_scene_on_slide_hotkey = value; return (Some(update), None)},
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
            StateUpdate::TimerCanRun(value) => {
                if self.timer_paused_length.is_some(){
                    return (None, Some(vec![StateUpdate::PauseTimer(false)]));
                }
                self.timer_can_run = value;
                self.timer_start = SystemTime::now();
                if value {
                    let mut instruction = Vec::new();
                    instruction.push(StateUpdate::TimerText(String::from("0.0")));
                    return (Some(update), Some(instruction))
                } else {
                    return (Some(update), None);
                }
            },
            StateUpdate::PauseTimer(value) => {return self.pause_timer(value)},
            StateUpdate::TimerLength(value) => {self.timer_length = value; return (Some(update), None)},
            StateUpdate::TimerText(value) => {self.timer_text = value.clone(); return (Some(StateUpdate::TimerText(value)), None)},
            StateUpdate::SubScene(value) => {
                if value.get_type().is_camera() {
                    if self.current_scene.is_camera() {
                        hotkey_handler.change_scene(Scenes::Camera, Some(value));
                    }
                    self.camera_sub_scene = value;
                    return (Some(update), None)
                } else if value.get_type().is_screen() {
                    if self.current_scene.is_screen() {
                        hotkey_handler.change_scene(Scenes::Screen, Some(value));
                    }
                    self.screen_sub_scene = value;
                    return (Some(update), None)
                }
            },
            StateUpdate::Scene(value) => {
                println!("handling scene: {:?}", value);
                
                if value.is_screen() {
                    if !self.current_scene.is_screen() {
                        self.timer_start = SystemTime::now();
                        self.timer_finished = false;
                    }
                } else {
                    self.timer_finished = true;
                }
                
                let mut instruction = None;
                if self.current_scene != value {
                    match value {
                        Scenes::Camera => {
                            hotkey_handler.change_scene(Scenes::Camera, Some(self.camera_sub_scene));
                            instruction = Some(vec![StateUpdate::TimerText("0.0".to_string())])
                        },
                        Scenes::Screen => {
                            hotkey_handler.change_scene(Scenes::Screen, Some(self.screen_sub_scene));
                        },
                        Scenes::Augmented => {
                            hotkey_handler.change_scene(Scenes::Augmented, None);
                        }
                    }
                }
                
                self.current_scene = value;
                return (Some(update), instruction);
            },
            StateUpdate::StreamSoundToggleOn(value) => {hotkey_handler.toggle_stream_sound(value); return (Some(update), None)},
            StateUpdate::ToggleComputerSoundOn(value) => {hotkey_handler.toggle_computer_sound(!value); return (Some(StateUpdate::ToggleComputerSoundOn(!value)), None)},
            StateUpdate::ComputerMediaDoPause => {hotkey_handler.toggle_media_play_pause(); return (Some(update), None)},
            StateUpdate::UpdateClient => {},
            StateUpdate::StreamRunning(_) => {},
            //_ => {}
        }
        (None, None)
    }

    fn pause_timer(&mut self, do_pause: bool) -> (Option<StateUpdate>, Option<Vec<StateUpdate>>) {
        let instruction: StateUpdate;
        
        // if do pause, 
        if do_pause {
            // stop tick from running,
            self.timer_can_run = false;
            
            // get the amount of time left on the clock
            let time_left: u16;
            match self.timer_start.elapsed() {
                Err(_) => {time_left = 0},
                Ok(change) => {
                    // take the duration left, multiply it by 10 to save the last decimal,
                    // then drop the rest of the digits with .round()
                    time_left = ((self.timer_length - change.as_secs_f32()) * 10.0).round() as u16;
                }
            }
            self.timer_paused_length = Some(time_left);
            
            // (Send to socket, process another instruction)
            // send the pause singal to the socket, and update the timer text (dividing by 10 to return the last digit)
            return (Some(StateUpdate::PauseTimer(true)), Some(vec![StateUpdate::TimerText(format!("{:.1}", time_left as f32/10.0))]));
        } else {
            // if start timer from pause
            // enable tick()
            self.timer_can_run = true;

            // Some fancy check to not have to use a match statement. The 'expect' should never be called, worry if it does
            let timer_paused_length: u16 = self.timer_paused_length.or(Some(0)).expect("timer_paused 'Some' unwrap somehow failed");
            
            // update timer_start, taking into account the amount of time already run
            self.timer_start = SystemTime::now() - 
                    std::time::Duration::from_millis(
                        // first get the decimal back from timer_paused_length, get the amount of time already run
                        // then convert that to milliseconds, then from f32 to u64 
                        ((self.timer_length - (timer_paused_length as f32 / 10.0)) * 1000.0) as u64);
                        
                
            // Clear the paused time
            self.timer_paused_length = None;

            instruction = StateUpdate::PauseTimer(false);
        }
        return (Some(instruction), None)
    }

    fn tick(&mut self) -> Vec<StateUpdate> {
        let mut instructions = Vec::new();
        if self.timer_finished == false && self.timer_can_run == true {
            match self.timer_start.elapsed() {
                Err(_) => {},
                Ok(change) => {
                    if change.as_secs_f32() >= self.timer_length {
                        self.timer_finished = true;
                        instructions.push(StateUpdate::TimerText(String::from("0.0")));
                        instructions.push(StateUpdate::Scene(Scenes::Camera));
                    } else {
                        instructions.push(StateUpdate::TimerText(
                            format!("{:.1}", self.timer_length - ((change.as_secs_f32() * 10.0).round() / 10.0))
                        ));
                    }
                }
            }
        }
        instructions
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
    let mut update = state.tick();
    state.update(update.pop().unwrap());
    assert_eq!(state.timer_text, "14.0");
}

#[test]
fn test_tick_one_half() {
    let mut state = StreamState::new();
    state.timer_finished = false;
    state.tick();
    std::thread::sleep(std::time::Duration::from_millis(500));
    let mut update = state.tick();
    state.update(update.pop().unwrap());
    assert_eq!(state.timer_text, "14.5");
}

#[test]
#[ignore]
fn test_tick_10() {
    let mut state = StreamState::new();
    state.timer_finished = false;
    state.tick();
    std::thread::sleep(std::time::Duration::from_millis(10000));
    let mut update = state.tick();
    state.update(update.pop().unwrap());
    assert_eq!(state.timer_text, "5.0");
}