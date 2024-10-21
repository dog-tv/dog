/// color
pub mod color;
/// frame
pub mod frame;
/// pixel renderable
pub mod pixel_renderable;
/// scene rendeable
pub mod scene_renderable;

use sophus::image::arc_image::ArcImage4U8;
use sophus::lie::Isometry3F64;

use crate::camera::RenderCamera;
use crate::renderables::frame::Frame;
use crate::renderables::pixel_renderable::PixelRenderable;
use crate::renderables::scene_renderable::SceneRenderable;

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

/// Create a image packet
pub fn make_image_packet(
    view_label: &str,
    frame: Option<Frame>,
    pixel_renderables: Vec<PixelRenderable>,
    scene_renderables: Vec<SceneRenderable>,
) -> Packet {
    Packet::Image(ImageViewPacket {
        frame,
        pixel_renderables,
        scene_renderables,
        view_label: view_label.to_owned(),
    })
}

/// Create a scene packet
pub fn make_scene_packet(
    view_label: &str,
    initial_camera: RenderCamera,
    scene_renderables: Vec<SceneRenderable>,
) -> Packet {
    Packet::Scene(SceneViewPacket {
        initial_camera,
        view_label: view_label.to_owned(),
        scene_renderables,
        locked_to_birds_eye_orientation: false,
        world_from_scene_update: None,
    })
}

/// Create a scene packet, which is locked to bird's eye view
pub fn make_birds_eye_scene_packet(
    view_label: &str,
    initial_camera: RenderCamera,
    scene_renderables: Vec<SceneRenderable>,
) -> Packet {
    Packet::Scene(SceneViewPacket {
        initial_camera,
        view_label: view_label.to_owned(),
        scene_renderables,
        locked_to_birds_eye_orientation: true,
        world_from_scene_update: None,
    })
}

/// Create world-from-scene update, scene packet
pub fn world_from_scene_update_packet(
    view_label: &str,
    world_from_scene_update: Isometry3F64,
) -> Packet {
    Packet::Scene(SceneViewPacket {
        initial_camera: RenderCamera::default(),
        view_label: view_label.to_owned(),
        scene_renderables: vec![],
        locked_to_birds_eye_orientation: false,
        world_from_scene_update: Some(world_from_scene_update),
    })
}

/// Packet to populate an image view
#[derive(Clone, Debug)]
pub struct ImageViewPacket {
    /// Frame to hold content
    ///
    ///  1. For each `view_label`, content (i.e. pixel_renderables, scene_renderables) will be added to
    ///     the existing frame. If no frame exists yet, e.g. frame was always None for `view_label`,
    ///     the content is ignored.
    ///  2. If we have a new frame, that is `frame == Some(...)`, all previous content is deleted, but
    ///     content from this packet will be added.
    pub frame: Option<Frame>,
    /// List of 2d renderables
    pub pixel_renderables: Vec<PixelRenderable>,
    /// List of scene renderables
    pub scene_renderables: Vec<SceneRenderable>,
    /// Name of the view
    pub view_label: String,
}

/// Packet to populate a scene view
#[derive(Clone, Debug)]
pub struct SceneViewPacket {
    /// List of 3d renderables
    pub scene_renderables: Vec<SceneRenderable>,
    /// world-from-scene pose update
    pub world_from_scene_update: Option<Isometry3F64>,
    /// Name of the view
    pub view_label: String,
    /// Initial camera, ignored if not the first packet for this view
    pub initial_camera: RenderCamera,
    /// lock xy plane
    pub locked_to_birds_eye_orientation: bool,
}
