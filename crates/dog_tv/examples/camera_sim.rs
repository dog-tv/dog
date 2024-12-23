#![cfg(feature = "std")]

use dog_tv::renderer::camera::properties::RenderCameraProperties;
use dog_tv::renderer::renderables::color::Color;
use dog_tv::renderer::renderables::scene_renderable::make_line3;
use dog_tv::renderer::renderables::scene_renderable::make_mesh3_at;
use dog_tv::renderer::renderables::scene_renderable::make_point3;
use dog_tv::renderer::RenderContext;
use dog_tv::sim::camera_simulator::CameraSimulator;
use dog_tv::sophus::image::io::png::save_as_png;
use dog_tv::sophus::image::io::tiff::save_as_tiff;
use dog_tv::sophus::image::ImageSize;
use dog_tv::sophus::lie::Isometry3;
use dog_tv::sophus::prelude::IsImageView;

pub async fn run_offscreen() {
    let render_state = RenderContext::new().await;

    let mut sim = CameraSimulator::new(
        &render_state,
        &RenderCameraProperties::default_from(ImageSize::new(639, 477)),
    );

    let mut renderables3d = vec![];
    let trig_points = [[0.0, 0.0, -0.1], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]];
    renderables3d.push(make_point3("points3", &trig_points, &Color::red(), 5.0));
    renderables3d.push(make_line3(
        "lines3",
        &[
            [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        ],
        &Color::green(),
        5.0,
    ));
    let blue = Color::blue();
    renderables3d.push(make_mesh3_at(
        "mesh",
        &[(trig_points, blue)],
        Isometry3::trans_z(3.0),
    ));

    sim.update_3d_renderables(renderables3d);

    let result = sim.render(Isometry3::trans_z(-5.0));

    save_as_png(&result.rgba_image.image_view(), "rgba.png").unwrap();

    let color_mapped_depth = result.depth_image.color_mapped();
    save_as_png(&color_mapped_depth.image_view(), "color_mapped_depth.png").unwrap();

    save_as_tiff(
        &result.depth_image.metric_depth().image_view(),
        "depth.tiff",
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
            run_offscreen().await;
        });
}
