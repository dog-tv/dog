use std::thread::spawn;

use dog_tv::examples::viewer_example::make_distorted_frame;
use dog_tv_renderer::camera::clipping_planes::ClippingPlanes;
use dog_tv_renderer::camera::properties::RenderCameraProperties;
use dog_tv_renderer::camera::RenderCamera;
use dog_tv_renderer::renderables::color::Color;
use dog_tv_renderer::renderables::pixel_renderable::make_point2;
use dog_tv_renderer::renderables::scene_renderable::make_point3;
use dog_tv_renderer::renderables::*;
use dog_tv_renderer::RenderContext;
use dog_tv_viewer::simple_viewer::SimpleViewer;
use sophus::core::linalg::VecF64;
use sophus::image::intensity_image::intensity_arc_image::IsIntensityArcImage;
use sophus::image::mut_image::MutImageF32;
use sophus::image::mut_image_view::IsMutImageView;
use sophus::image::ImageSize;
use sophus::lie::prelude::IsVector;
use sophus::lie::Isometry3;
use sophus::sensor::dyn_camera::DynCameraF64;

use crate::frame::Frame;
use crate::pixel_renderable::make_line2;
use crate::plot::scalar_curve::ScalarCurveStyle;
use crate::plot::LineType;
use crate::scene_renderable::make_line3;
use crate::scene_renderable::make_mesh3_at;

fn create_distorted_image_packet() -> Packet {
    let mut image_packet = ImageViewPacket {
        view_label: "distorted image".to_owned(),
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
        view_label: "tiny image".to_owned(),
        scene_renderables: vec![],
        pixel_renderables: vec![],
        frame: Some(Frame::from_image(&img.to_shared().to_rgba())),
    };

    image_packet.pixel_renderables.push(make_line2(
        "lines2",
        &[[[-0.5, -0.5], [0.5, 0.5]], [[0.5, -0.5], [-0.5, 0.5]]],
        &Color::red(),
        2.0,
    ));

    Packet::Image(image_packet)
}

fn create_scene_packet(pinhole: bool) -> Packet {
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

    let mut scene_packet = SceneViewPacket {
        view_label: match pinhole {
            false => "scene - distorted".to_owned(),
            true => "scene - bird's eye".to_owned(),
        },
        scene_renderables: vec![],
        initial_camera: initial_camera.clone(),
        locked_to_birds_eye_orientation: pinhole,
        world_from_scene_update: None,
    };

    let trig_points = [[0.0, 0.0, -0.1], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]];

    scene_packet
        .scene_renderables
        .push(make_point3("points3", &trig_points, &Color::red(), 5.0));

    scene_packet.scene_renderables.push(make_line3(
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
    scene_packet.scene_renderables.push(make_mesh3_at(
        "mesh",
        &[(trig_points, blue)],
        Isometry3::trans_z(3.0),
    ));

    Packet::Scene(scene_packet)
}

pub fn run_viewer_example() {
    let (message_tx, message_rx) = std::sync::mpsc::channel();

    spawn(move || {
        let mut packets = Packets { packets: vec![] };
        packets.packets.push(create_scene_packet(true));
        packets.packets.push(create_scene_packet(false));
        packets.packets.push(create_distorted_image_packet());
        packets.packets.push(create_tiny_image_view_packet());
        message_tx.send(packets).unwrap();

        let mut x: f64 = 0.0;

        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));

            let sin_x = x.sin();

            let plot_packets = vec![PlotViewPacket::append_to_curve(
                ("trig0", "sin"),
                (x, sin_x),
                ScalarCurveStyle {
                    color: Color::red(),
                    line_type: LineType::default(),
                },
            )];

            let mut packets = Packets { packets: vec![] };
            packets
                .packets
                .push(Packet::Plot(plot_packets));
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
