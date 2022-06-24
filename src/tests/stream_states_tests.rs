use std::sync::mpsc;
use std::thread;

use crate::modules::stream_states as s_s;
use crate::modules::stream_states::state_update::StateUpdate;

#[test]
fn has_all_enums() {
    {
        let members = [
            s_s::scenes::SubScenes::CameraDefault,
            s_s::scenes::SubScenes::CameraWithUpperRight,
            s_s::scenes::SubScenes::CameraWithLowerRight,
            s_s::scenes::SubScenes::CameraWithLargeUpperRight,
        ];
        assert_eq!(members.len(), 4);
    }
    {
        let members = [
            s_s::scenes::SubScenes::ScreenDefault,
            s_s::scenes::SubScenes::ScreenWithUpperRight,
            s_s::scenes::SubScenes::ScreenWithLowerRight,
        ];
        assert_eq!(members.len(), 3);
    }
}

#[test]
fn create_stream_states_class() {
    let stream_state = s_s::stream_states_class::StreamState::new();
    assert_eq!(stream_state.stream_running, false);
    assert_eq!(stream_state.stream_is_muted, false);
    assert_eq!(stream_state.computer_sound_is_on, true);
    assert_eq!(stream_state.change_scene_on_slide_hotkey, true);
    assert_eq!(stream_state.scene_is_augmented, false);

    assert_eq!(stream_state.timer_text, "0.0");
    assert_eq!(stream_state.timer_length, 15.0);
    assert_eq!(stream_state.timer_can_run, true);

    assert_eq!(stream_state.current_scene, s_s::scenes::Scenes::Camera);
    assert_eq!(stream_state.camera_sub_scene, s_s::scenes::SubScenes::CameraDefault);
    assert_eq!(stream_state.screen_sub_scene, s_s::scenes::SubScenes::ScreenDefault);

}

#[test]
fn scene_correctness(){
    let mut stream_state = s_s::stream_states_class::StreamState::new();
    assert_eq!(stream_state.current_scene, s_s::scenes::Scenes::Camera);
    assert_eq!(stream_state.camera_sub_scene, s_s::scenes::SubScenes::CameraDefault);
    assert_eq!(stream_state.screen_sub_scene, s_s::scenes::SubScenes::ScreenDefault);

    stream_state.update(StateUpdate::SubScene(s_s::scenes::SubScenes::CameraWithUpperRight));

    assert_eq!(stream_state.camera_sub_scene, s_s::scenes::SubScenes::CameraWithUpperRight);
    assert_eq!(stream_state.screen_sub_scene, s_s::scenes::SubScenes::ScreenDefault);

    stream_state.update(StateUpdate::SubScene(s_s::scenes::SubScenes::CameraWithLargeUpperRight));

    assert_eq!(stream_state.camera_sub_scene, s_s::scenes::SubScenes::CameraWithLargeUpperRight);
    assert_eq!(stream_state.screen_sub_scene, s_s::scenes::SubScenes::ScreenDefault);

    stream_state.update(StateUpdate::SubScene(s_s::scenes::SubScenes::CameraWithLowerRight));

    assert_eq!(stream_state.camera_sub_scene, s_s::scenes::SubScenes::CameraWithLowerRight);
    assert_eq!(stream_state.screen_sub_scene, s_s::scenes::SubScenes::ScreenDefault);

    stream_state.update(StateUpdate::SubScene(s_s::scenes::SubScenes::ScreenDefault));
    stream_state.update(StateUpdate::Scene(s_s::scenes::Scenes::Screen));
    
    assert_eq!(stream_state.current_scene, s_s::scenes::Scenes::Screen);
    assert_eq!(stream_state.screen_sub_scene, s_s::scenes::SubScenes::ScreenDefault);
    assert_eq!(stream_state.camera_sub_scene, s_s::scenes::SubScenes::CameraWithLowerRight);

    stream_state.update(StateUpdate::SubScene(s_s::scenes::SubScenes::ScreenWithLowerRight));
    
    assert_eq!(stream_state.screen_sub_scene, s_s::scenes::SubScenes::ScreenWithLowerRight);
    assert_eq!(stream_state.camera_sub_scene, s_s::scenes::SubScenes::CameraWithLowerRight);

    stream_state.update(StateUpdate::SubScene(s_s::scenes::SubScenes::ScreenWithUpperRight));
    
    assert_eq!(stream_state.screen_sub_scene, s_s::scenes::SubScenes::ScreenWithUpperRight);
    assert_eq!(stream_state.camera_sub_scene, s_s::scenes::SubScenes::CameraWithLowerRight);

    stream_state.update(StateUpdate::Scene(s_s::scenes::Scenes::Augmented));

    assert_eq!(stream_state.current_scene, s_s::scenes::Scenes::Augmented);
    assert_eq!(stream_state.screen_sub_scene, s_s::scenes::SubScenes::ScreenWithUpperRight);
    assert_eq!(stream_state.camera_sub_scene, s_s::scenes::SubScenes::CameraWithLowerRight);

}

#[test]
fn test_updating() {
    let mut stream_state = s_s::stream_states_class::StreamState::new();
    
    assert_eq!(stream_state.timer_can_run, true);
    stream_state.update(StateUpdate::TimerCanRun(false));
    assert_eq!(stream_state.timer_can_run, false);

    assert_eq!(stream_state.timer_length, 15.0);
    stream_state.update(StateUpdate::TimerLength(7.5));
    assert_eq!(stream_state.timer_length, 7.5);

    assert_eq!(stream_state.timer_text, "0.0");
    stream_state.update(StateUpdate::TimerText(String::from("7.5")));
    assert_eq!(stream_state.timer_text, "7.5");

    assert_eq!(stream_state.stream_running, false);
    stream_state.update(StateUpdate::StreamRunning(true));
    assert_eq!(stream_state.stream_running, true);

    assert_eq!(stream_state.stream_is_muted, false);
    stream_state.update(StateUpdate::StreamSoundToggleOn(true));
    assert_eq!(stream_state.stream_is_muted, true);

    assert_eq!(stream_state.computer_sound_is_on, true);
    stream_state.update(StateUpdate::ToggleComputerSoundOn(false));
    assert_eq!(stream_state.computer_sound_is_on, false);

    assert_eq!(stream_state.change_scene_on_slide_hotkey, true);
    stream_state.update(StateUpdate::ChangeSceneOnChangeSlide(false));
    assert_eq!(stream_state.change_scene_on_slide_hotkey, false);

    assert_eq!(stream_state.scene_is_augmented, false);
    stream_state.update(StateUpdate::SceneIsAugmented(true));
    assert_eq!(stream_state.scene_is_augmented, true);
}


#[test]
fn can_run_in_thread() {
    let (tx, rx) = mpsc::channel();

    let rx_thread = thread::spawn(move || {
        let mut stream_state = s_s::stream_states_class::StreamState::new();
        for received in rx {
            assert_eq!(received, StateUpdate::StreamRunning(true));
            stream_state.update(received);
            assert_eq!(stream_state.stream_running, true);
            break;
        }
    });

    tx.send(StateUpdate::StreamRunning(true)).unwrap();
    let result = rx_thread.join();
    match result {
        Ok(_) => return,
        Err(_) => panic!(),
    }
}