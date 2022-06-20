use std::str::FromStr;


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

impl SubScenes {
    pub fn to_string(&self) -> String {
        match self {
            SubScenes::CameraDefault        => {String::from_str("Camera_None").unwrap()},
            SubScenes::CameraWithUpperRight => {String::from_str("Camera_Top_Right").unwrap()},
            SubScenes::CameraWithLargeUpperRight => {String::from_str("Camera_Bottom_Left").unwrap()},
            SubScenes::CameraWithLowerRight => {String::from_str("Camera_Bottom_Right").unwrap()},
            SubScenes::ScreenDefault        => {String::from_str("Screen_None").unwrap()},
            SubScenes::ScreenWithUpperRight => {String::from_str("Screen_Top_Right").unwrap()},
            SubScenes::ScreenWithLowerRight => {String::from_str("Screen_Bottom_Right").unwrap()},
        }
    }

    pub fn get_type(&self) -> Scenes {
        match self {
            SubScenes::CameraDefault | SubScenes::CameraWithUpperRight |
            SubScenes::CameraWithLargeUpperRight | SubScenes::CameraWithLowerRight => {
                Scenes::Camera
            }
            SubScenes::ScreenDefault | SubScenes::ScreenWithUpperRight |
            SubScenes::ScreenWithLowerRight => {
                Scenes::Screen
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Scenes {
    Camera,
    Screen,
    Augmented
}

impl Scenes {
    pub fn to_string(&self) -> String {
        match self {
            Scenes::Camera => {String::from_str("Scene_Camera").unwrap()},
            Scenes::Screen => {String::from_str("Scene_Screen").unwrap()},
            Scenes::Augmented => {String::from_str("Augmented").unwrap()},
        }
    }
    
    pub fn is_camera(&self) -> bool {
        match self { Scenes::Camera => true, _ => false }
    }

    pub fn is_screen(&self) -> bool {
        match self { Scenes::Screen => true, _ => false }
    }

    pub fn is_augmeted(&self) -> bool {
        match self {Scenes::Augmented => true, _ => false }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SlideChange {
    NextApp,
    PreviousApp,
    NextHotkey,
    PreviousHotkey,
}
