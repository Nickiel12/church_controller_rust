use crate::modules::stream_states::{state_update::StateUpdate, enums::{Scenes, SubScenes, SlideChange}};



#[test]
fn test_json_to_state_update() {
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
            "{\"type\": \"update\", \"update\": \"Scene\", \"data\": \"Scene_Camera\"}"
        ).unwrap()), StateUpdate::Scene(Scenes::Camera));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
            "{\"type\": \"update\", \"update\": \"Scene\", \"data\": \"Scene_Screen\"}"
        ).unwrap()), StateUpdate::Scene(Scenes::Screen));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
            "{\"type\": \"update\", \"update\": \"Scene_Is_Augmented\", \"data\": \"true\"}"
        ).unwrap()), StateUpdate::SceneIsAugmented(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Timer_Can_Run\", \"data\": \"true\"}"
        ).unwrap()), StateUpdate::TimerCanRun(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Timer_Can_Run\", \"data\": \"false\"}"
        ).unwrap()), StateUpdate::TimerCanRun(false));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Change_With_Clicker\", \"data\": \"true\"}"
        ).unwrap()), StateUpdate::ChangeSceneOnChangeSlide(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Change_With_Clicker\", \"data\": \"false\"}"
        ).unwrap()), StateUpdate::ChangeSceneOnChangeSlide(false));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Toggle_Computer_Volume\", \"data\": \"true\"}"
        ).unwrap()), StateUpdate::ComputerSoundIsOn(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Toggle_Computer_Volume\", \"data\": \"false\"}"
        ).unwrap()), StateUpdate::ComputerSoundIsOn(false));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Toggle_Stream_Volume\", \"data\": \"true\"}"
        ).unwrap()), StateUpdate::StreamIsMuted(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Toggle_Stream_Volume\", \"data\": \"false\"}"
        ).unwrap()), StateUpdate::StreamIsMuted(false));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Media_Pause_Play\", \"data\": \"true\"}"
        ).unwrap()), StateUpdate::ComputerMediaDoPause(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Media_Pause_Play\", \"data\": \"false\"}"
        ).unwrap()), StateUpdate::ComputerMediaDoPause(false));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"SubScene\", \"data\": \"Camera_None\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::CameraDefault));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"SubScene\", \"data\": \"Camera_Top_Right\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::CameraWithUpperRight));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"SubScene\", \"data\": \"Camera_Bottom_Right\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::CameraWithLowerRight));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"SubScene\", \"data\": \"Camera_Bottom_Left\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::CameraWithLargeUpperRight));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"SubScene\", \"data\": \"Screen_None\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::ScreenDefault));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"SubScene\", \"data\": \"Screen_Top_Right\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::ScreenWithUpperRight));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"SubScene\", \"data\":\"Screen_Bottom_Right\"}"
        ).unwrap()), StateUpdate::SubScene(SubScenes::ScreenWithLowerRight));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Timer_Length\", \"data\": 5.5}"
        ).unwrap()), StateUpdate::TimerLength(5.5));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Next_Slide\"}"
        ).unwrap()), StateUpdate::ChangeSlide(SlideChange::Next));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Prev_Slide\"}"
        ).unwrap()), StateUpdate::ChangeSlide(SlideChange::Previous));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\":\"all\"}"
        ).unwrap()), StateUpdate::UpdateClient);
}

#[test]
#[should_panic]
fn test_json_to_state_update_fails() {
    StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"AnUnknownType\"}"
    ).unwrap());
}

#[test]
fn test_state_update_to_json() {
    println!("{:?}", StateUpdate::StreamRunning(true).to_json());
    //Note, this one needs to is dependant on test_json_to_update for correctedness
    assert_eq!(StateUpdate::StreamRunning(true), (StateUpdate::json_to_state_update(StateUpdate::StreamRunning(true).to_json())));
}