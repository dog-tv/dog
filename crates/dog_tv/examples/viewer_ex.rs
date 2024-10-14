use dog_tv::examples::viewer_example::make_distorted_frame;
use dog_tv_renderer::camera::clipping_planes::ClippingPlanes;
use dog_tv_renderer::camera::properties::RenderCameraProperties;
use dog_tv_renderer::camera::RenderCamera;
use dog_tv_renderer::renderables::color::Color;
use dog_tv_renderer::renderables::renderable2d::make_point2;
use dog_tv_renderer::renderables::renderable3d::make_point3;
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
use crate::renderable2d::make_line2;
use crate::renderable3d::make_line3;
use crate::renderable3d::make_mesh3_at;

fn create_distorted_image_packet() -> Packet {
    let mut packet_2d = ImageViewPacket {
        view_label: "distorted image".to_owned(),
        renderables3d: vec![],
        renderables2d: vec![],
        frame: Some(make_distorted_frame()),
    };

    packet_2d.renderables2d.push(make_point2(
        "points2",
        &[[16.0, 12.0], [32.0, 24.0]],
        &Color::red(),
        5.0,
    ));
    packet_2d.renderables2d.push(make_line2(
        "lines2",
        &[[[0.0, 0.0], [20.0, 20.0]]],
        &Color::blue(),
        5.0,
    ));

    packet_2d.renderables3d.push(make_line3(
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

    Packet::Image(packet_2d)
}

fn create_tiny_image_view_packet() -> Packet {
    let mut img = MutImageF32::from_image_size_and_val(ImageSize::new(3, 2), 1.0);

    *img.mut_pixel(0, 0) = 0.0;
    *img.mut_pixel(0, 1) = 0.5;

    *img.mut_pixel(1, 1) = 0.0;

    *img.mut_pixel(2, 0) = 0.3;
    *img.mut_pixel(2, 1) = 0.6;

    let mut packet_2d = ImageViewPacket {
        view_label: "tiny image".to_owned(),
        renderables3d: vec![],
        renderables2d: vec![],
        frame: Some(Frame::from_image(&img.to_shared().to_rgba())),
    };

    packet_2d.renderables2d.push(make_line2(
        "lines2",
        &[[[-0.5, -0.5], [0.5, 0.5]], [[0.5, -0.5], [-0.5, 0.5]]],
        &Color::red(),
        2.0,
    ));

    Packet::Image(packet_2d)
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

    let mut packet_3d = SceneViewPacket {
        view_label: match pinhole {
            false => "scene - distorted".to_owned(),
            true => "scene - bird's eye".to_owned(),
        },
        renderables3d: vec![],
        initial_camera: initial_camera.clone(),
        locked_to_birds_eye_orientation: pinhole,
    };

    let trig_points = [[0.0, 0.0, -0.1], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]];

    packet_3d
        .renderables3d
        .push(make_point3("points3", &trig_points, &Color::red(), 5.0));

    packet_3d.renderables3d.push(make_line3(
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
    packet_3d.renderables3d.push(make_mesh3_at(
        "mesh",
        &[(trig_points, blue)],
        Isometry3::trans_z(3.0),
    ));

    Packet::Scene(packet_3d)
}

pub async fn run_viewer_example() {
    let (message_tx, message_rx) = std::sync::mpsc::channel();

    tokio::spawn(async move {
        let mut packets = Packets { packets: vec![] };
        packets.packets.push(create_scene_packet(true));
        packets.packets.push(create_scene_packet(false));
        packets.packets.push(create_distorted_image_packet());
        packets.packets.push(create_tiny_image_view_packet());
        message_tx.send(packets).unwrap();
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
    env_logger::init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            run_viewer_example().await;
        })
}
