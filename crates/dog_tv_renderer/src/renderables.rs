/// color
pub mod color;
/// frame
pub mod frame;
/// pixel renderable
pub mod pixel_renderable;
/// plot
pub mod plot;
/// scene rendeable
pub mod scene_renderable;

use crate::camera::RenderCamera;
use crate::renderables::frame::Frame;
use crate::renderables::pixel_renderable::PixelRenderable;
use crate::renderables::plot::curve_vec_with_conf::CurveVecWithConf;
use crate::renderables::plot::curve_vec_with_conf::CurveVecWithConfStyle;
use crate::renderables::plot::curve_vec_with_conf::NamedVecConfCurve;
use crate::renderables::plot::scalar_curve::NamedScalarCurve;
use crate::renderables::plot::scalar_curve::ScalarCurve;
use crate::renderables::plot::scalar_curve::ScalarCurveStyle;
use crate::renderables::plot::vec_curve::CurveVec;
use crate::renderables::plot::vec_curve::CurveVecStyle;
use crate::renderables::plot::vec_curve::NamedCurveVec;
use crate::renderables::plot::ClearCondition;
use crate::renderables::scene_renderable::SceneRenderable;
use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use sophus::image::arc_image::ArcImage4U8;
use sophus::lie::Isometry3F64;

extern crate alloc;

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
    /// plot view packet
    Plot(Vec<PlotViewPacket>),
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
        view_label: view_label.to_string(),
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
        view_label: view_label.to_string(),
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
        view_label: view_label.to_string(),
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
        view_label: view_label.to_string(),
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

/// Packet to populate a scene view
#[derive(Clone, Debug)]
pub enum PlotViewPacket {
    /// a float value
    Scalar(NamedScalarCurve),
    /// a 2d vector curve
    Vec2(NamedCurveVec<2>),
    /// a 2d vector curve with confidence intervals
    Vec2Conf(NamedVecConfCurve<2>),
    /// a 3d vector curve
    Vec3(NamedCurveVec<3>),
    /// a 3d vector curve with confidence intervals
    Vec3Conf(NamedVecConfCurve<3>),
}

impl PlotViewPacket {
    /// Get the name of the plot
    pub fn name(&self) -> String {
        match self {
            PlotViewPacket::Scalar(named_scalar_curve) => named_scalar_curve.plot_name.clone(),
            PlotViewPacket::Vec2(named_vec_curve) => named_vec_curve.plot_name.clone(),
            PlotViewPacket::Vec2Conf(named_vec_conf_curve) => {
                named_vec_conf_curve.plot_name.clone()
            }
            PlotViewPacket::Vec3(named_vec_curve) => named_vec_curve.plot_name.clone(),
            PlotViewPacket::Vec3Conf(named_vec_conf_curve) => {
                named_vec_conf_curve.plot_name.clone()
            }
        }
    }
}

impl PlotViewPacket {
    /// Append data to a curve
    pub fn append_to_curve<S: Into<String>>(
        (plot, graph): (S, S),
        data: VecDeque<(f64, f64)>,
        style: ScalarCurveStyle,
        clear_cond: ClearCondition,
        v_line: Option<f64>,
    ) -> PlotViewPacket {
        let curve = NamedScalarCurve {
            plot_name: plot.into(),
            graph_name: graph.into(),
            scalar_curve: ScalarCurve {
                data,
                style,
                clear_cond,
                v_line,
            },
        };
        PlotViewPacket::Scalar(curve)
    }

    /// Append data to a 2-vector of curves
    pub fn append_to_curve_vec2<S: Into<String>>(
        (plot, graph): (S, S),
        data: VecDeque<(f64, [f64; 2])>,
        style: CurveVecStyle<2>,
        clear_cond: ClearCondition,
        v_line: Option<f64>,
    ) -> PlotViewPacket {
        let curve = NamedCurveVec {
            plot_name: plot.into(),
            curve_name: graph.into(),
            scalar_curve: CurveVec {
                data,
                style,
                clear_cond,
                v_line,
            },
        };

        PlotViewPacket::Vec2(curve)
    }

    /// Append data to a 3-vector of curves
    pub fn append_to_curve_vec3<S: Into<String>>(
        (plot, graph): (S, S),
        data: VecDeque<(f64, [f64; 3])>,
        style: CurveVecStyle<3>,
        clear_cond: ClearCondition,
        v_line: Option<f64>,
    ) -> PlotViewPacket {
        let curve = NamedCurveVec {
            plot_name: plot.into(),
            curve_name: graph.into(),
            scalar_curve: CurveVec {
                data,
                style,
                clear_cond,
                v_line,
            },
        };

        PlotViewPacket::Vec3(curve)
    }

    /// Append data to a 2-vector of curves with confidence intervals
    pub fn append_to_curve_vec2_with_conf<S: Into<String>>(
        (plot, graph): (S, S),
        data: plot::curve_vec_with_conf::DataVecDeque<2>,
        style: CurveVecWithConfStyle<2>,
        clear_cond: ClearCondition,
        v_line: Option<f64>,
    ) -> PlotViewPacket {
        let curve = NamedVecConfCurve {
            plot_name: plot.into(),
            curve_name: graph.into(),
            scalar_curve: CurveVecWithConf {
                data,
                style,
                clear_cond,
                v_line,
            },
        };

        PlotViewPacket::Vec2Conf(curve)
    }

    /// Append data to a 3-vector of curves with confidence intervals
    pub fn append_to_curve_vec3_with_conf<S: Into<String>>(
        (plot, graph): (S, S),
        data: plot::curve_vec_with_conf::DataVecDeque<3>,
        style: CurveVecWithConfStyle<3>,
        clear_cond: ClearCondition,
        v_line: Option<f64>,
    ) -> PlotViewPacket {
        let curve = NamedVecConfCurve {
            plot_name: plot.into(),
            curve_name: graph.into(),
            scalar_curve: CurveVecWithConf {
                data,
                style,
                clear_cond,
                v_line,
            },
        };

        PlotViewPacket::Vec3Conf(curve)
    }
}
