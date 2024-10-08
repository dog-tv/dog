use crate::interactions::orbit_interaction::OrbitalInteraction;
use crate::interactions::InteractionEnum;
use crate::views::View;
use dog_tv_renderer::aspect_ratio::HasAspectRatio;
use dog_tv_renderer::camera::intrinsics::RenderIntrinsics;
use dog_tv_renderer::offscreen_renderer::OffscreenRenderer;
use dog_tv_renderer::renderables::renderable3d::View3dPacket;
use dog_tv_renderer::RenderContext;
use linked_hash_map::LinkedHashMap;

pub(crate) struct View3d {
    pub(crate) renderer: OffscreenRenderer,
    pub(crate) interaction: InteractionEnum,
    pub(crate) enabled: bool,
    pub(crate) lock_xy_plane: bool,
}

impl View3d {
    fn create_if_new(
        views: &mut LinkedHashMap<String, View>,
        packet: &View3dPacket,
        state: &RenderContext,
    ) {
        if !views.contains_key(&packet.view_label) {
            views.insert(
                packet.view_label.clone(),
                View::View3d(View3d {
                    renderer: OffscreenRenderer::new(state, &packet.initial_camera.properties),
                    interaction: InteractionEnum::Orbital(OrbitalInteraction::new(
                        &packet.view_label,
                        packet.initial_camera.scene_from_camera,
                        packet.initial_camera.properties.clipping_planes,
                    )),
                    enabled: true,
                    lock_xy_plane: packet.lock_xy_plane,
                }),
            );
        }
    }

    pub fn update(
        views: &mut LinkedHashMap<String, View>,
        packet: View3dPacket,
        state: &RenderContext,
    ) {
        Self::create_if_new(views, &packet, state);

        let view = views.get_mut(&packet.view_label).unwrap();
        let view = match view {
            View::View3d(view) => view,
            _ => panic!("View type mismatch"),
        };

        view.renderer.update_3d_renderables(packet.renderables3d);
    }

    pub fn intrinsics(&self) -> RenderIntrinsics {
        self.renderer.intrinsics()
    }
}

impl HasAspectRatio for View3d {
    fn aspect_ratio(&self) -> f32 {
        self.renderer.aspect_ratio()
    }
}
