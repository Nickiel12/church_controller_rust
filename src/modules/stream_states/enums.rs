
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SubScenes {
    CameraDefault,
    CameraWithUpperRight,
    CameraWithLargeUpperRight,
    CameraWithLowerRight,

    ScreenDefault,
    ScreenWithUpperRight,
    ScreenWithLowerRight,

}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Scenes {
    Camera,
    Screen,
    Augmented
}

