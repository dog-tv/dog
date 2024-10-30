#![cfg(feature = "std")]

use core::f64::consts::TAU;
use dog_tv::examples::viewer_example::make_distorted_frame;
use dog_tv::renderer::camera::clipping_planes::ClippingPlanes;
use dog_tv::renderer::camera::properties::RenderCameraProperties;
use dog_tv::renderer::camera::RenderCamera;
use dog_tv::renderer::renderables::color::Color;
use dog_tv::renderer::renderables::frame::ImageFrame;
use dog_tv::renderer::renderables::pixel_renderable::make_line2;
use dog_tv::renderer::renderables::pixel_renderable::make_point2;
use dog_tv::renderer::renderables::scene_renderable::make_line3;
use dog_tv::renderer::renderables::scene_renderable::make_mesh3_at;
use dog_tv::renderer::renderables::scene_renderable::make_point3;
use dog_tv::renderer::RenderContext;
use dog_tv::sophus::core::linalg::VecF64;
use dog_tv::sophus::image::intensity_image::intensity_arc_image::IsIntensityArcImage;
use dog_tv::sophus::image::mut_image::MutImageF32;
use dog_tv::sophus::image::mut_image_view::IsMutImageView;
use dog_tv::sophus::image::ImageSize;
use dog_tv::sophus::lie::prelude::IsVector;
use dog_tv::sophus::lie::Isometry3;
use dog_tv::sophus::sensor::dyn_camera::DynCameraF64;
use dog_tv::thingbuf::mpsc::blocking::channel;
use dog_tv::viewer::packets::append_to_scene_packet;
use dog_tv::viewer::packets::create_scene_packet;
use dog_tv::viewer::packets::image_view_packet::ImageViewPacket;
use dog_tv::viewer::packets::plot_view_packet::curve_vec_with_conf::CurveVecWithConfStyle;
use dog_tv::viewer::packets::plot_view_packet::scalar_curve::ScalarCurveStyle;
use dog_tv::viewer::packets::plot_view_packet::vec_curve::CurveVecStyle;
use dog_tv::viewer::packets::plot_view_packet::ClearCondition;
use dog_tv::viewer::packets::plot_view_packet::LineType;
use dog_tv::viewer::packets::plot_view_packet::PlotViewPacket;
use dog_tv::viewer::packets::Packet;
use dog_tv::viewer::simple_viewer::SimpleViewer;
use std::thread::spawn;

fn create_distorted_image_packet() -> Packet {
    let mut image_packet = ImageViewPacket {
        view_label: "distorted image".to_string(),
        scene_renderables: vec![],
        pixel_renderables: vec![],
        frame: Some(make_distorted_frame()),
    };

    image_packet.pixel_renderables.push(make_point2(
        "points2",
        &[[16.0, 12.0], [32.0, 24.0]],
        &Color::red(),
        5.0,
    ));
    image_packet.pixel_renderables.push(make_line2(
        "lines2",
        &[[[0.0, 0.0], [20.0, 20.0]]],
        &Color::blue(),
        5.0,
    ));

    image_packet.scene_renderables.push(make_line3(
        "lines3",
        &[
            [[-0.5, -0.3, 1.0], [-0.5, 0.3, 1.0]],
            [[-0.5, 0.3, 1.0], [0.5, 0.3, 1.0]],
            [[0.5, 0.3, 1.0], [0.5, -0.3, 1.0]],
            [[0.5, -0.3, 1.0], [-0.5, -0.3, 1.0]],
        ],
        &Color::green(),
        5.0,
    ));

    Packet::Image(image_packet)
}

fn create_tiny_image_view_packet() -> Packet {
    let mut img = MutImageF32::from_image_size_and_val(ImageSize::new(3, 2), 1.0);

    *img.mut_pixel(0, 0) = 0.0;
    *img.mut_pixel(0, 1) = 0.5;

    *img.mut_pixel(1, 1) = 0.0;

    *img.mut_pixel(2, 0) = 0.3;
    *img.mut_pixel(2, 1) = 0.6;

    let mut image_packet = ImageViewPacket {
        view_label: "tiny image".to_string(),
        scene_renderables: vec![],
        pixel_renderables: vec![],
        frame: Some(ImageFrame::from_image(&img.to_shared().to_rgba())),
    };

    image_packet.pixel_renderables.push(make_line2(
        "lines2",
        &[[[-0.5, -0.5], [0.5, 0.5]], [[0.5, -0.5], [-0.5, 0.5]]],
        &Color::red(),
        2.0,
    ));

    Packet::Image(image_packet)
}

fn create_scene(pinhole: bool) -> Vec<Packet> {
    let unified_cam = DynCameraF64::new_unified(
        &VecF64::from_array([500.0, 500.0, 320.0, 240.0, 0.629, 1.02]),
        ImageSize::new(639, 479),
    );
    let pinhole_cam = DynCameraF64::new_pinhole(
        &VecF64::from_array([500.0, 500.0, 320.0, 240.0]),
        ImageSize::new(639, 479),
    );

    let initial_camera = RenderCamera {
        properties: RenderCameraProperties::new(
            match pinhole {
                true => pinhole_cam,
                false => unified_cam,
            },
            ClippingPlanes::default(),
        ),
        scene_from_camera: Isometry3::trans_z(-5.0),
    };

    let mut scene_renderables = vec![];
    let trig_points = [[0.0, 0.0, -0.1], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]];
    scene_renderables.push(make_point3("points3", &trig_points, &Color::red(), 5.0));

    scene_renderables.push(make_line3(
        "lines3",
        &[
            [[0.0, 0.1, 0.0], [0.1, 0.2, 0.0]],
            [[0.1, 0.2, 0.0], [0.2, 0.3, 0.0]],
            [[0.2, 0.3, 0.0], [0.3, 0.4, 0.0]],
            [[0.3, 0.4, 0.0], [0.4, 0.5, 0.0]],
            [[0.4, 0.5, 0.0], [0.5, 0.6, 0.0]],
            [[0.5, 0.6, 0.0], [0.6, 0.7, 0.0]],
            [[0.6, 0.7, 0.0], [0.7, 0.8, 0.0]],
            [[0.7, 0.8, 0.0], [0.8, 0.9, 0.0]],
            [[0.8, 0.9, 0.0], [0.9, 1.0, 0.0]],
            [[0.9, 1.0, 0.0], [1.0, 1.1, 0.0]],
        ],
        &Color::green(),
        5.0,
    ));

    let blue = Color::blue();
    scene_renderables.push(make_mesh3_at(
        "mesh",
        &[(trig_points, blue)],
        Isometry3::trans_z(3.0),
    ));

    let label = match pinhole {
        false => "scene - distorted",
        true => "scene - bird's eye",
    };
    let packets = vec![
        create_scene_packet(label, initial_camera, pinhole),
        append_to_scene_packet(label, scene_renderables),
    ];

    packets
}

pub fn run_viewer_example() {
    let (message_tx, message_rx) = channel(50);

    spawn(move || {
        let mut packets = vec![];
        packets.append(&mut create_scene(true));
        packets.append(&mut create_scene(false));
        packets.push(create_distorted_image_packet());
        packets.push(create_tiny_image_view_packet());
        message_tx.send(packets).unwrap();

        let mut x: f64 = 0.0;

        loop {
            std::thread::sleep(std::time::Duration::from_millis(10));

            let sin_x = x.sin();
            let cos_x = x.cos();
            let tan_x = x.tan().clamp(-1.5, 1.5);

            let plot_packets = vec![
                PlotViewPacket::append_to_curve(
                    ("scalar-curve", "sin"),
                    vec![(x, sin_x)].into(),
                    ScalarCurveStyle {
                        color: Color::orange(),
                        line_type: LineType::default(),
                    },
                    ClearCondition { max_x_range: TAU },
                    Some(x - 0.2),
                ),
                PlotViewPacket::append_to_curve_vec3(
                    ("curve-vec", ("sin_cos_tan")),
                    vec![(x, [sin_x, cos_x, tan_x])].into(),
                    CurveVecStyle {
                        colors: [Color::red(), Color::green(), Color::blue()],
                        line_type: LineType::default(),
                    },
                    ClearCondition { max_x_range: TAU },
                    Some(x - 0.2),
                ),
                PlotViewPacket::append_to_curve_vec2_with_conf(
                    ("curve-vec +- e", ("sin_cos")),
                    vec![(x, ([sin_x, cos_x], [0.1 * sin_x, 0.1 * sin_x]))].into(),
                    CurveVecWithConfStyle {
                        colors: [Color::red(), Color::green()],
                    },
                    ClearCondition { max_x_range: TAU },
                    Some(x - 0.2),
                ),
            ];

            let packets = vec![Packet::Plot(plot_packets)];
            message_tx.send(packets).unwrap();

            x += 0.01;
        }
    });

    eframe::run_native(
        "Viewer Example",
        dog_tv_viewer::recommened_eframe_native_options(),
        Box::new(|cc| {
            Ok(SimpleViewer::new(
                RenderContext::from_egui_cc(cc),
                message_rx,
            ))
        }),
    )
    .unwrap();
}

fn main() {
    run_viewer_example();
}
