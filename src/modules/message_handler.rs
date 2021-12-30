use std::time::{SystemTime, Duration};

use super::{stream_states::{state_update::StateUpdate, stream_states_class::StreamState, enums::{SlideChange, Scenes}}, external_interface, socket_handler::Socket};

pub trait MessageHandler {
    fn handle_update(&mut self, update: StateUpdate) -> Option<StateUpdate>;
    fn get_states(&self) -> StreamState;
    fn tick(&mut self) -> (Option<StateUpdate>, Option<StateUpdate>);
}

impl MessageHandler for StreamState {
    fn handle_update(&mut self, update: StateUpdate) -> Option<StateUpdate> {
        self.update(update.clone());

        match update {
            StateUpdate::ChangeSlide(direction) => {
                if self.timer_can_run {
                    self.timer_finished = false;
                    self.timer_start = SystemTime::now();
                }
                if self.change_scene_on_change_slide_hotkey {
                    self.handle_update(StateUpdate::Scene(Scenes::Screen));
                }
                match direction {
                    SlideChange::Next => {
                        external_interface::next_slide();
                    },
                    SlideChange::Previous => {
                        external_interface::prev_slide();
                    }
                }
            }
            _ => {}
        }
        None
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
    std::thread::sleep(Duration::from_millis(1000));
    state.tick();
    assert_eq!(state.timer_text, "14.0");
}

#[test]
fn test_tick_one_half() {
    let mut state = StreamState::new();
    state.timer_finished = false;
    state.tick();
    std::thread::sleep(Duration::from_millis(500));
    state.tick();
    assert_eq!(state.timer_text, "14.5");
}

#[test]
#[ignore]
fn test_tick_10() {
    let mut state = StreamState::new();
    state.timer_finished = false;
    state.tick();
    std::thread::sleep(Duration::from_millis(10000));
    state.tick();
    assert_eq!(state.timer_text, "5.0");
}