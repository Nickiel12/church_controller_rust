use std::time::{SystemTime, Duration};

use super::{stream_states::{state_update::StateUpdate, stream_states_class::StreamState, enums::{SlideChange, Scenes}}, external_interface};

pub trait MessageHandler {
    fn handle_update(&mut self, update: StateUpdate) -> ();
    fn get_states(&self) -> StreamState;
    fn tick(&mut self) -> ();
}

impl MessageHandler for StreamState {
    fn handle_update(&mut self, update: StateUpdate) {
        self.update(update.clone());

        match update {
            StateUpdate::ChangeSlide(direction) => {
                if self.timer_can_run {
                    self.timer_finished = false;
                    self.timer_start = SystemTime::now();
                    self.tick();
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
    }

    fn tick(&mut self) {
        if self.timer_finished == false {
            let change = self.timer_start.elapsed();
            match change {
                Err(_) => {},
                Ok(change) => {
                    if change.as_secs_f32() >= self.timer_length {
                        self.handle_update(StateUpdate::TimerText(String::from("0.0")));
                        self.handle_update(StateUpdate::Scene(Scenes::Camera));
                        self.timer_finished = true;
                    } else {
                        self.handle_update(StateUpdate::TimerText(
                            format!("{:.1}", self.timer_length - ((change.as_secs_f32() * 10.0).round() / 10.0))
                        ));
                    }
                }
            };
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