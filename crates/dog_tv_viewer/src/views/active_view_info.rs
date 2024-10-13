use dog_tv_renderer::camera::properties::RenderCameraProperties;
use dog_tv_renderer::types::SceneFocusMarker;
use sophus::image::ImageSize;
use sophus::lie::Isometry3F64;

/// active view info
pub struct ActiveViewInfo {
    /// name of active view
    pub active_view: String,
    /// scene-from-camera pose
    pub scene_from_camera: Isometry3F64,
    /// camere properties
    pub camera_properties: RenderCameraProperties,
    /// scene focus
    pub scene_focus: SceneFocusMarker,
    /// type
    pub view_type: String,
    /// view-port size
    pub view_port_size: ImageSize,
    /// xy-locked
    pub locked_to_birds_eye_view: bool,
}
