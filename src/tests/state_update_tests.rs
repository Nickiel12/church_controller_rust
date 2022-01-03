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
        ).unwrap()), StateUpdate::ToggleComputerSoundOn(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Toggle_Computer_Volume\", \"data\": \"false\"}"
        ).unwrap()), StateUpdate::ToggleComputerSoundOn(false));
    
    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Toggle_Stream_Volume\", \"data\": \"true\"}"
        ).unwrap()), StateUpdate::StreamSoundToggleOn(true));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Toggle_Stream_Volume\", \"data\": \"false\"}"
        ).unwrap()), StateUpdate::StreamSoundToggleOn(false));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Media_Pause_Play\", \"data\": \"true\"}"
        ).unwrap()), StateUpdate::ComputerMediaDoPause);

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
        "{\"type\": \"update\", \"update\": \"Timer_Length\", \"data\": \"5.5\"}"
        ).unwrap()), StateUpdate::TimerLength(5.5));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Next_Slide\"}"
        ).unwrap()), StateUpdate::ChangeSlide(SlideChange::NextHotkey));

    assert_eq!(StateUpdate::json_to_state_update(serde_json::from_str(
        "{\"type\": \"update\", \"update\": \"Prev_Slide\"}"
        ).unwrap()), StateUpdate::ChangeSlide(SlideChange::PreviousHotkey));

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
    //Note, this one needs to is dependant on test_json_to_update for correctedness
    assert_eq!(StateUpdate::StreamRunning(true), (StateUpdate::json_to_state_update(StateUpdate::StreamRunning(true).to_json())));
    assert_eq!(StateUpdate::StreamRunning(false), StateUpdate::json_to_state_update(StateUpdate::StreamRunning(false).to_json()));
    assert_eq!(StateUpdate::StreamSoundToggleOn(true), StateUpdate::json_to_state_update(StateUpdate::StreamSoundToggleOn(true).to_json()));
    assert_eq!(StateUpdate::StreamSoundToggleOn(false), StateUpdate::json_to_state_update(StateUpdate::StreamSoundToggleOn(false).to_json()));
    assert_eq!(StateUpdate::ToggleComputerSoundOn(true), StateUpdate::json_to_state_update(StateUpdate::ToggleComputerSoundOn(true).to_json()));
    assert_eq!(StateUpdate::ToggleComputerSoundOn(false), StateUpdate::json_to_state_update(StateUpdate::ToggleComputerSoundOn(false).to_json()));
    assert_eq!(StateUpdate::ChangeSceneOnChangeSlide(true), StateUpdate::json_to_state_update(StateUpdate::ChangeSceneOnChangeSlide(true).to_json()));
    assert_eq!(StateUpdate::ChangeSceneOnChangeSlide(false), StateUpdate::json_to_state_update(StateUpdate::ChangeSceneOnChangeSlide(false).to_json()));
    assert_eq!(StateUpdate::SceneIsAugmented(true), StateUpdate::json_to_state_update(StateUpdate::SceneIsAugmented(true).to_json()));
    assert_eq!(StateUpdate::SceneIsAugmented(false), StateUpdate::json_to_state_update(StateUpdate::SceneIsAugmented(false).to_json()));
    assert_eq!(StateUpdate::TimerCanRun(true), StateUpdate::json_to_state_update(StateUpdate::TimerCanRun(true).to_json()));
    assert_eq!(StateUpdate::TimerCanRun(false), StateUpdate::json_to_state_update(StateUpdate::TimerCanRun(false).to_json()));
    assert_eq!(StateUpdate::TimerLength(17.5), StateUpdate::json_to_state_update(StateUpdate::TimerLength(17.5).to_json()));
    assert_eq!(StateUpdate::TimerText(String::from("15.6")), StateUpdate::json_to_state_update(StateUpdate::TimerText(String::from("15.6")).to_json()));
    assert_eq!(StateUpdate::SubScene(SubScenes::CameraDefault), StateUpdate::json_to_state_update(StateUpdate::SubScene(SubScenes::CameraDefault).to_json()));
    assert_eq!(StateUpdate::SubScene(SubScenes::CameraWithUpperRight), StateUpdate::json_to_state_update(StateUpdate::SubScene(SubScenes::CameraWithUpperRight).to_json()));
    assert_eq!(StateUpdate::SubScene(SubScenes::CameraWithLowerRight), StateUpdate::json_to_state_update(StateUpdate::SubScene(SubScenes::CameraWithLowerRight).to_json()));
    assert_eq!(StateUpdate::SubScene(SubScenes::CameraWithLargeUpperRight), StateUpdate::json_to_state_update(StateUpdate::SubScene(SubScenes::CameraWithLargeUpperRight).to_json()));
    assert_eq!(StateUpdate::SubScene(SubScenes::ScreenDefault), StateUpdate::json_to_state_update(StateUpdate::SubScene(SubScenes::ScreenDefault).to_json()));
    assert_eq!(StateUpdate::SubScene(SubScenes::ScreenWithUpperRight), StateUpdate::json_to_state_update(StateUpdate::SubScene(SubScenes::ScreenWithUpperRight).to_json()));
    assert_eq!(StateUpdate::SubScene(SubScenes::ScreenWithLowerRight), StateUpdate::json_to_state_update(StateUpdate::SubScene(SubScenes::ScreenWithLowerRight).to_json()));
    assert_eq!(StateUpdate::Scene(Scenes::Camera), StateUpdate::json_to_state_update(StateUpdate::Scene(Scenes::Camera).to_json()));
    assert_eq!(StateUpdate::Scene(Scenes::Screen), StateUpdate::json_to_state_update(StateUpdate::Scene(Scenes::Screen).to_json()));
    assert_eq!(StateUpdate::ChangeSlide(SlideChange::NextHotkey), StateUpdate::json_to_state_update(StateUpdate::ChangeSlide(SlideChange::NextHotkey).to_json()));
    assert_eq!(StateUpdate::ChangeSlide(SlideChange::PreviousHotkey), StateUpdate::json_to_state_update(StateUpdate::ChangeSlide(SlideChange::PreviousHotkey).to_json()));
    assert_eq!(StateUpdate::UpdateClient, StateUpdate::json_to_state_update(StateUpdate::UpdateClient.to_json()));

}