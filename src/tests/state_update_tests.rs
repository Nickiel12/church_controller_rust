use crate::modules::stream_states::{state_update::StateUpdate, enums::{Scenes, SubScenes}};



#[test]
fn test_json_to_state_update() {
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
            "{\"type\": \"button\", \"button\": \"Scene_Camera\"}"
        ).unwrap()), StateUpdate::Scene(Scenes::Camera));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
            "{\"type\": \"button\", \"button\": \"Scene_Screen\"}"
        ).unwrap()), StateUpdate::Scene(Scenes::Screen));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
            "{\"type\": \"button\", \"button\": \"Augmented\", \"data\": true}"
        ).unwrap()), StateUpdate::SceneIsAugmented(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Auto_Change_To_Camera\", \"data\": true}"
        ).unwrap()), StateUpdate::TimerCanRun(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Auto_Change_To_Camera\", \"data\": false}"
        ).unwrap()), StateUpdate::TimerCanRun(false));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Change_With_Clicker\", \"data\": true}"
        ).unwrap()), StateUpdate::ChangeSceneOnChangeSlideHotkey(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Change_With_Clicker\", \"data\": false}"
        ).unwrap()), StateUpdate::ChangeSceneOnChangeSlideHotkey(false));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Toggle_Computer_Volume\", \"data\": true}"
        ).unwrap()), StateUpdate::ComputerSoundIsOn(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Toggle_Computer_Volume\", \"data\": false}"
        ).unwrap()), StateUpdate::ComputerSoundIsOn(false));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Toggle_Stream_Volume\", \"data\": true}"
        ).unwrap()), StateUpdate::StreamIsMuted(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Toggle_Stream_Volume\", \"data\": false}"
        ).unwrap()), StateUpdate::StreamIsMuted(false));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Media_Pause_Play\", \"data\": true}"
        ).unwrap()), StateUpdate::ComputerMediaDoPause(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Media_Pause_Play\", \"data\": false}"
        ).unwrap()), StateUpdate::ComputerMediaDoPause(false));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Camera_None\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::CameraDefault));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Camera_Top_Right\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::CameraWithUpperRight));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Camera_Bottom_Right\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::CameraWithLowerRight));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Camera_Bottom_Left\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::CameraWithLargeUpperRight));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Screen_None\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::ScreenDefault));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Screen_Top_Right\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::ScreenWithUpperRight));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"button\", \"button\": \"Screen_Bottom_Right\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::ScreenWithLowerRight));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"Timer_Length\", \"data\": 5.5}"
    ).unwrap()), StateUpdate::TimerLength(5.5));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\"}"
    ).unwrap()), StateUpdate::UpdateClient);
}

#[test]
#[should_panic]
fn test_json_to_state_update_fails() {
    StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"AnUnknownType\"}"
    ).unwrap());
}