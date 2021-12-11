
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Scenes {
    CameraDefault,
    CameraWithUpperRight,
    CameraWithLargeUpperRight,
    CameraWithLowerRight,

    ScreenDefault,
    ScreenWithUpperRight,
    ScreenWithLowerRight,

    Augmented
}

#[derive(Debug, PartialEq, Clone)]
pub enum StateUpdate {
    StreamRunning(bool),
    StreamIsMuted(bool),
    ComputerSoundIsOn(bool),
    ChangeSceneOnChangeSlideHotkey(bool),
    SceneIsAugmented(bool),
    TimerCanRun(bool),
    TimerLength(f32),
    TimerText(String),
    Scene(Scenes),
}