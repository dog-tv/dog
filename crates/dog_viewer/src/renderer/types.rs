use eframe::egui;
use sophus::core::linalg::VecF64;
use sophus::image::arc_image::ArcImage4U8;

use crate::renderer::textures::depth_image::DepthImage;
use crate::renderer::OffscreenRenderer;
use crate::viewer::aspect_ratio::HasAspectRatio;

/// Render result
pub struct RenderResult {
    /// rgba image
    pub rgba_image: Option<ArcImage4U8>,

    /// rgba egui texture id
    pub rgba_egui_tex_id: egui::TextureId,

    /// depth image
    pub depth_image: DepthImage,

    /// depth egui texture id
    pub depth_egui_tex_id: egui::TextureId,
}

impl HasAspectRatio for OffscreenRenderer {
    fn aspect_ratio(&self) -> f32 {
        self.camera_properties
            .intrinsics
            .image_size()
            .aspect_ratio()
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Zoom2dPod {
    pub(crate) translation_x: f32,
    pub(crate) translation_y: f32,
    pub(crate) scaling_x: f32,
    pub(crate) scaling_y: f32,
}

impl Default for Zoom2dPod {
    fn default() -> Self {
        Zoom2dPod {
            translation_x: 0.0,
            translation_y: 0.0,
            scaling_x: 1.0,
            scaling_y: 1.0,
        }
    }
}

/// Translation and scaling
///
/// todo: move to sophus::lie
#[derive(Clone, Copy, Debug)]
pub struct TranslationAndScaling {
    /// translation
    pub translation: VecF64<2>,
    /// scaling
    pub scaling: VecF64<2>,
}

impl TranslationAndScaling {
    /// identity
    pub fn identity() -> Self {
        TranslationAndScaling {
            translation: VecF64::<2>::zeros(),
            scaling: VecF64::<2>::new(1.0, 1.0),
        }
    }

    /// apply translation and scaling
    pub fn apply(&self, xy: VecF64<2>) -> VecF64<2> {
        VecF64::<2>::new(
            xy[0] * self.scaling[0] + self.translation[0],
            xy[1] * self.scaling[1] + self.translation[1],
        )
    }
}