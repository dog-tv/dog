/// color
pub mod color;
/// frame
pub mod frame;
/// 2d renderable
pub mod renderable2d;
/// 3d rendeable
pub mod renderable3d;

use sophus::image::arc_image::ArcImage4U8;
use sophus::lie::Isometry3F64;

use crate::camera::RenderCamera;
use crate::renderables::frame::Frame;
use crate::renderables::renderable2d::Renderable2d;
use crate::renderables::renderable3d::Renderable3d;

/// Image view renderable
#[derive(Clone, Debug)]
pub enum ImageViewRenderable {
    /// Background image
    BackgroundImage(ArcImage4U8),
}

/// Packet of renderables
#[derive(Clone, Debug)]
pub enum Packet {
    /// scene view packet
    Scene(SceneViewPacket),
    /// image view packet
    Image(ImageViewPacket),
}

/// Packet of renderables
#[derive(Clone, Debug, Default)]
pub struct Packets {
    /// List of packets
    pub packets: Vec<Packet>,
}

/// Create a view3d packet
pub fn make_view2d_packet(
    view_label: &str,
    frame: Option<Frame>,
    renderables2d: Vec<Renderable2d>,
    renderables3d: Vec<Renderable3d>,
) -> Packet {
    Packet::Image(ImageViewPacket {
        frame,
        renderables2d,
        renderables3d,
        view_label: view_label.to_owned(),
    })
}

/// Create a view3d packet
pub fn make_view3d_packet(
    view_label: &str,
    initial_camera: RenderCamera,
    renderables3d: Vec<Renderable3d>,
) -> Packet {
    Packet::Scene(SceneViewPacket {
        initial_camera,
        view_label: view_label.to_owned(),
        renderables3d,
        locked_to_birds_eye_orientation: false,
        world_from_scene_update: None,
    })
}

/// Create a view3d packet
pub fn make_view3d_packet_xy_locked(
    view_label: &str,
    initial_camera: RenderCamera,
    renderables3d: Vec<Renderable3d>,
) -> Packet {
    Packet::Scene(SceneViewPacket {
        initial_camera,
        view_label: view_label.to_owned(),
        renderables3d,
        locked_to_birds_eye_orientation: true,
        world_from_scene_update: None,
    })
}

/// Create world-from-scene update packet
pub fn world_from_scene_update_packet(
    view_label: &str,
    world_from_scene_update: Isometry3F64,
) -> Packet {
    Packet::Scene(SceneViewPacket {
        initial_camera: RenderCamera::default(),
        view_label: view_label.to_owned(),
        renderables3d: vec![],
        locked_to_birds_eye_orientation: false,
        world_from_scene_update: Some(world_from_scene_update),
    })
}

/// Packet to populate an image view
#[derive(Clone, Debug)]
pub struct ImageViewPacket {
    /// Frame to hold content
    ///
    ///  1. For each `view_label`, content (i.e. renderables2d, renderables3d) will be added to
    ///     the existing frame. If no frame exists yet, e.g. frame was always None for `view_label`,
    ///     the content is ignored.
    ///  2. If we have a new frame, that is `frame == Some(...)`, all previous content is deleted, but
    ///     content from this packet will be added.
    pub frame: Option<Frame>,
    /// List of 2d renderables
    pub renderables2d: Vec<Renderable2d>,
    /// List of 3d renderables
    pub renderables3d: Vec<Renderable3d>,
    /// Name of the view
    pub view_label: String,
}

/// Packet to populate a scene view
#[derive(Clone, Debug)]
pub struct SceneViewPacket {
    /// List of 3d renderables
    pub renderables3d: Vec<Renderable3d>,
    /// world-from-scene pose update
    pub world_from_scene_update: Option<Isometry3F64>,
    /// Name of the view
    pub view_label: String,
    /// Initial camera, ignored if not the first packet for this view
    pub initial_camera: RenderCamera,
    /// lock xy plane
    pub locked_to_birds_eye_orientation: bool,
}
