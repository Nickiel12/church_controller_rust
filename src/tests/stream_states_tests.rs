
use crate::modules::stream_states as s_s;

#[test]
fn has_all_enums() {
    {
        let members = [
            s_s::enums::CameraScenes::CameraDefault,
            s_s::enums::CameraScenes::CameraWithUpperRight,
            s_s::enums::CameraScenes::CameraWithLowerRight,
            s_s::enums::CameraScenes::CameraWithLargeUpperRight,
        ];
        assert_eq!(members.len(), 4);
    }
    {
        let members = [
            s_s::enums::ScreenScenes::ScreenDefault,
            s_s::enums::ScreenScenes::ScreenWithUpperRight,
            s_s::enums::ScreenScenes::ScreenWithLowerRight,
        ];
        assert_eq!(members.len(), 3);
    }

}