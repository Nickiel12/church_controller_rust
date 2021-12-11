
use crate::modules::stream_states as s_s;

#[test]
fn has_all_enums() {
    {
        let members = [
            s_s::enums::Scenes::CameraDefault,
            s_s::enums::Scenes::CameraWithUpperRight,
            s_s::enums::Scenes::CameraWithLowerRight,
            s_s::enums::Scenes::CameraWithLargeUpperRight,
        ];
        assert_eq!(members.len(), 4);
    }
    {
        let members = [
            s_s::enums::Scenes::ScreenDefault,
            s_s::enums::Scenes::ScreenWithUpperRight,
            s_s::enums::Scenes::ScreenWithLowerRight,
        ];
        assert_eq!(members.len(), 3);
    }
}

#[test]
fn create_stream_states_class() {
    let stream_state = s_s::stream_states_class::StreamStates::new();
    assert_eq!(stream_state.stream_running, false);
    assert_eq!(stream_state.stream_is_muted, false);
    assert_eq!(stream_state.computer_sound_is_on, true);
    assert_eq!(stream_state.change_scene_on_change_slide_hotkey, true);
    assert_eq!(stream_state.scene_is_augmented, false);

    assert_eq!(stream_state.timer_text, "0.0");
    assert_eq!(stream_state.timer_length, 15.0);
    assert_eq!(stream_state.timer_can_run, true);

    assert_eq!(stream_state.current_scene, s_s::enums::Scenes::CameraDefault);
    assert_eq!(stream_state.camera_sub_scene, s_s::enums::Scenes::CameraDefault);
    assert_eq!(stream_state.screen_sub_scene, s_s::enums::Scenes::ScreenDefault);

}

#[test]
fn scene_correctness(){
    let mut stream_state = s_s::stream_states_class::StreamStates::new();
    assert_eq!(stream_state.current_scene, s_s::enums::Scenes::CameraDefault);
    assert_eq!(stream_state.camera_sub_scene, s_s::enums::Scenes::CameraDefault);
    assert_eq!(stream_state.screen_sub_scene, s_s::enums::Scenes::ScreenDefault);

    stream_state = stream_state.change_scene(&s_s::enums::Scenes::CameraWithUpperRight);

    assert_eq!(stream_state.current_scene, s_s::enums::Scenes::CameraWithUpperRight);
    assert_eq!(stream_state.camera_sub_scene, s_s::enums::Scenes::CameraWithUpperRight);
    assert_eq!(stream_state.screen_sub_scene, s_s::enums::Scenes::ScreenDefault);

    stream_state = stream_state.change_scene(&s_s::enums::Scenes::CameraWithLargeUpperRight);

    assert_eq!(stream_state.current_scene, s_s::enums::Scenes::CameraWithLargeUpperRight);
    assert_eq!(stream_state.camera_sub_scene, s_s::enums::Scenes::CameraWithLargeUpperRight);
    assert_eq!(stream_state.screen_sub_scene, s_s::enums::Scenes::ScreenDefault);

    stream_state = stream_state.change_scene(&s_s::enums::Scenes::CameraWithLowerRight);

    assert_eq!(stream_state.current_scene, s_s::enums::Scenes::CameraWithLowerRight);
    assert_eq!(stream_state.camera_sub_scene, s_s::enums::Scenes::CameraWithLowerRight);
    assert_eq!(stream_state.screen_sub_scene, s_s::enums::Scenes::ScreenDefault);

    stream_state = stream_state.change_scene(&s_s::enums::Scenes::ScreenDefault);
    
    assert_eq!(stream_state.current_scene, s_s::enums::Scenes::ScreenDefault);
    assert_eq!(stream_state.screen_sub_scene, s_s::enums::Scenes::ScreenDefault);
    assert_eq!(stream_state.camera_sub_scene, s_s::enums::Scenes::CameraWithLowerRight);

    stream_state = stream_state.change_scene(&s_s::enums::Scenes::ScreenWithLowerRight);
    
    assert_eq!(stream_state.current_scene, s_s::enums::Scenes::ScreenWithLowerRight);
    assert_eq!(stream_state.screen_sub_scene, s_s::enums::Scenes::ScreenWithLowerRight);
    assert_eq!(stream_state.camera_sub_scene, s_s::enums::Scenes::CameraWithLowerRight);

    stream_state = stream_state.change_scene(&s_s::enums::Scenes::ScreenWithUpperRight);
    
    assert_eq!(stream_state.current_scene, s_s::enums::Scenes::ScreenWithUpperRight);
    assert_eq!(stream_state.screen_sub_scene, s_s::enums::Scenes::ScreenWithUpperRight);
    assert_eq!(stream_state.camera_sub_scene, s_s::enums::Scenes::CameraWithLowerRight);

}