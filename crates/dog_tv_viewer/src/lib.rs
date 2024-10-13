#![deny(missing_docs)]
#![allow(clippy::needless_range_loop)]

//! Simple viewer for 2D and 3D visualizations.

/// Interactions
pub mod interactions;
/// eframea app impl
pub mod simple_viewer;
/// Viewer base
pub mod viewer_base;
/// The view struct.
pub mod views;

/// eframe native options - recommended for use with the dog-tv
pub fn recommened_eframe_native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([850.0, 480.0]),
        renderer: eframe::Renderer::Wgpu,
        multisampling: dog_tv_renderer::types::DOG_MULTISAMPLE_COUNT as u16,
        ..Default::default()
    }
}
