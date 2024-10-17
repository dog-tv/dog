use crate::interactions::orbit_interaction::OrbitalInteraction;
use crate::interactions::InteractionEnum;
use crate::views::View;
use dog_tv_renderer::aspect_ratio::HasAspectRatio;
use dog_tv_renderer::camera::intrinsics::RenderIntrinsics;
use dog_tv_renderer::offscreen_renderer::OffscreenRenderer;
use dog_tv_renderer::renderables::SceneViewPacket;
use dog_tv_renderer::RenderContext;
use linked_hash_map::LinkedHashMap;
use sophus::lie::Isometry3F64;

pub(crate) struct SceneView {
    pub(crate) renderer: OffscreenRenderer,
    pub(crate) interaction: InteractionEnum,
    pub(crate) enabled: bool,
    pub(crate) locked_to_birds_eye_orientation: bool,
}

impl SceneView {
    fn create_if_new(
        views: &mut LinkedHashMap<String, View>,
        packet: &SceneViewPacket,
        state: &RenderContext,
    ) {
        if !views.contains_key(&packet.view_label) {
            views.insert(
                packet.view_label.clone(),
                View::Scene(SceneView {
                    renderer: OffscreenRenderer::new(state, &packet.initial_camera.properties),
                    interaction: InteractionEnum::Orbital(OrbitalInteraction::new(
                        &packet.view_label,
                        packet.initial_camera.scene_from_camera,
                        packet.initial_camera.properties.clipping_planes,
                    )),
                    enabled: true,
                    locked_to_birds_eye_orientation: packet.locked_to_birds_eye_orientation,
                }),
            );
        }
    }

    pub fn update(
        views: &mut LinkedHashMap<String, View>,
        packet: SceneViewPacket,
        state: &RenderContext,
    ) {
        Self::create_if_new(views, &packet, state);

        let view = views.get_mut(&packet.view_label).unwrap();
        let scene_view = match view {
            View::Scene(view) => view,
            _ => panic!("View type mismatch"),
        };
        if let Some(world_from_scene_update) = packet.world_from_scene_update {
            scene_view.renderer.scene.world_from_scene = world_from_scene_update;
        }
        scene_view
            .renderer
            .update_3d_renderables(packet.renderables3d);
    }

    pub fn intrinsics(&self) -> RenderIntrinsics {
        self.renderer.intrinsics()
    }
}

impl HasAspectRatio for SceneView {
    fn aspect_ratio(&self) -> f32 {
        self.renderer.aspect_ratio()
    }
}
