use crate::interactions::ViewportScale;
use crate::views::active_view_info::ActiveViewInfo;
use crate::views::get_adjusted_view_size;
use crate::views::get_max_size;
use crate::views::image_view::ImageView;
use crate::views::scene_view::SceneView;
use crate::views::View;
use dog_tv_renderer::aspect_ratio::HasAspectRatio;
use dog_tv_renderer::renderables::Packet;
use dog_tv_renderer::renderables::Packets;
use dog_tv_renderer::RenderContext;
use eframe::egui;
use linked_hash_map::LinkedHashMap;
use sophus::image::arc_image::ArcImageF32;
use sophus::image::ImageSize;
use sophus::prelude::HasParams;
use sophus::prelude::IsTranslationProductGroup;
use std::collections::HashMap;

/// Viewer top-level struct.
pub struct ViewerBase {
    state: RenderContext,
    views: LinkedHashMap<String, View>,
    message_recv: std::sync::mpsc::Receiver<Packets>,
    show_depth: bool,
    backface_culling: bool,
    responses: HashMap<String, ResponseStruct>,
    active_view: String,
    active_view_info: Option<ActiveViewInfo>,
}

pub(crate) struct ResponseStruct {
    pub(crate) ui_response: egui::Response,
    pub(crate) z_image: ArcImageF32,
    pub(crate) scales: ViewportScale,
    pub(crate) view_port_size: ImageSize,
}

impl ViewerBase {
    /// Create a new viewer.
    pub fn new(
        render_state: RenderContext,
        message_recv: std::sync::mpsc::Receiver<Packets>,
    ) -> ViewerBase {
        ViewerBase {
            state: render_state.clone(),
            views: LinkedHashMap::new(),
            message_recv,
            show_depth: false,
            backface_culling: false,
            responses: HashMap::new(),
            active_view_info: None,
            active_view: Default::default(),
        }
    }

    /// Update the data.
    pub fn update_data(&mut self) {
        self.add_renderables_to_tables();
    }

    /// Process events.
    pub fn process_events(&mut self) {
        for (view_label, view) in self.views.iter_mut() {
            let mut view_port_size = ImageSize::default();
            match view {
                View::Scene(view) => {
                    if let Some(response) = self.responses.get(view_label) {
                        view.interaction.process_event(
                            &mut self.active_view,
                            &view.intrinsics(),
                            view.locked_to_birds_eye_orientation,
                            &response.ui_response,
                            &response.scales,
                            response.view_port_size,
                            &response.z_image,
                        );
                        view_port_size = response.view_port_size
                    }
                }
                View::Image(view) => {
                    if let Some(response) = self.responses.get(view_label) {
                        view.interaction.process_event(
                            &mut self.active_view,
                            &view.intrinsics(),
                            true,
                            &response.ui_response,
                            &response.scales,
                            response.view_port_size,
                            &response.z_image,
                        );
                        view_port_size = response.view_port_size
                    }
                }
            }

            if view.interaction().is_active() && &self.active_view == view_label {
                self.active_view_info = Some(ActiveViewInfo {
                    active_view: view_label.clone(),
                    scene_from_camera: view.interaction().scene_from_camera(),
                    camera_properties: Some(view.camera_propterties()),
                    // is_active, so marker is guaranteed to be Some
                    scene_focus: view.interaction().marker().unwrap(),
                    view_type: view.view_type(),
                    view_port_size,
                    locked_to_birds_eye_orientation: view.locked_to_birds_eye_orientation(),
                });
            }
        }
        self.responses.clear();
    }

    /// Update bottom status bar
    pub fn update_top_bar(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            egui::CollapsingHeader::new("Settings").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.show_depth, "show depth");
                    ui.checkbox(&mut self.backface_culling, "backface culling");
                });
            });

            let help_button_response = ui.button("❓");

            let popup_id = ui.make_persistent_id("help");
            if help_button_response.clicked() {
                ui.memory_mut(|mem| mem.toggle_popup(popup_id));
            }
            let below = egui::AboveOrBelow::Below;
            let close_on_click_outside = egui::popup::PopupCloseBehavior::CloseOnClickOutside;
            egui::popup::popup_above_or_below_widget(
                ui,
                popup_id,
                &help_button_response,
                below,
                close_on_click_outside,
                |ui| {
                    ui.set_width(250.0);
                    ui.label("PAN UP/DOWN + LEFT/RIGHT");
                    ui.label("mouse: left-button drag");
                    ui.label("touchpad: one finger drag");
                    ui.label("");
                    ui.label("ROTATE UP/DOWN + LEFT/RIGHT*");
                    ui.label("mouse: right-button drag");
                    ui.label("touchpad: two finger drag** / shift + drag");
                    ui.label("");
                    ui.label("ZOOM");
                    ui.label("mouse: scroll-wheel");
                    ui.label("touchpad: two finger vertical scroll");
                    ui.label("");
                    ui.label("ROTATE IN-PLANE");
                    ui.label("mouse: shift + scroll-wheel");
                    ui.label("touchpad: two finger horizontal scroll");
                    ui.label("");
                    ui.label("* Disabled if locked to birds-eye orientation.");
                    ui.label("** Does not work on all touchpads.");
                },
            );
        });
    }

    /// Update the left panel.
    pub fn update_left_panel(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        for (view_label, view) in self.views.iter_mut() {
            ui.checkbox(view.enabled_mut(), view_label);
        }
        ui.separator();
    }

    /// Update bottom status bar
    pub fn update_bottom_status_bar(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        match self.active_view_info.as_ref() {
            Some(view_info) => {
                if let Some(camera_properties) = view_info.camera_properties.as_ref() {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(format!(
                            "{}: {}, view-port: {} x {}, image: {} x {}, clip: [{}, {}], \
                            focus uv: {:0.1} {:0.1}, ndc-z: {:0.3}, metric-z: {:0.3}",
                            view_info.view_type,
                            view_info.active_view,
                            view_info.view_port_size.width,
                            view_info.view_port_size.height,
                            camera_properties.intrinsics.image_size().width,
                            camera_properties.intrinsics.image_size().height,
                            camera_properties.clipping_planes.near,
                            camera_properties.clipping_planes.far,
                            view_info.scene_focus.u,
                            view_info.scene_focus.v,
                            view_info.scene_focus.ndc_z,
                            camera_properties
                                .clipping_planes
                                .metric_z_from_ndc_z(view_info.scene_focus.ndc_z as f64),
                        ));

                        let scene_from_camera_orientation = view_info.scene_from_camera.rotation();
                        let scene_from_camera_quaternion = scene_from_camera_orientation.params();

                        ui.label(format!(
                            "CAMERA position: ({:0.3}, {:0.3}, {:0.3}), quaternion: {:0.4}, \
                            ({:0.4}, {:0.4}, {:0.4}), bird's eye view: {}",
                            view_info.scene_from_camera.translation()[0],
                            view_info.scene_from_camera.translation()[1],
                            view_info.scene_from_camera.translation()[2],
                            scene_from_camera_quaternion[0],
                            scene_from_camera_quaternion[1],
                            scene_from_camera_quaternion[2],
                            scene_from_camera_quaternion[3],
                            view_info.locked_to_birds_eye_orientation
                        ));
                    });
                } else {
                    ui.label(format!(
                        "{}: {}, view-port: {} x {}",
                        view_info.view_type,
                        view_info.active_view,
                        view_info.view_port_size.width,
                        view_info.view_port_size.height,
                    ));
                }
            }
            None => {
                ui.label("view: n/a");
            }
        }
    }

    /// Update the central panel.
    pub fn update_central_panel(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.scope(|ui0| {
            if self.views.is_empty() {
                return;
            }
            let maybe_max_size = get_max_size(
                &self.views,
                0.99 * ui0.available_width(),
                0.99 * ui0.available_height(),
            );
            if maybe_max_size.is_none() {
                return;
            }
            let (max_width, max_height) = maybe_max_size.unwrap();

            ui0.horizontal_wrapped(|ui| {
                for (view_label, view) in self.views.iter_mut() {
                    if !view.enabled() {
                        continue;
                    }

                    let view_aspect_ratio = view.aspect_ratio();
                    let adjusted_size =
                        get_adjusted_view_size(view_aspect_ratio, max_width, max_height);
                    match view {
                        View::Scene(view) => {
                            let render_result = view
                                .renderer
                                .render_params(
                                    &adjusted_size.image_size(),
                                    &view.interaction.scene_from_camera(),
                                )
                                .zoom(view.interaction.zoom2d())
                                .interaction(view.interaction.marker())
                                .backface_culling(self.backface_culling)
                                .compute_depth_texture(self.show_depth)
                                .render();

                            let egui_texture = if self.show_depth {
                                render_result.depth_egui_tex_id
                            } else {
                                render_result.rgba_egui_tex_id
                            };

                            let ui_response = ui.add(
                                egui::Image::new(egui::load::SizedTexture {
                                    size: egui::Vec2::new(
                                        adjusted_size.width,
                                        adjusted_size.height,
                                    ),
                                    id: egui_texture,
                                })
                                .fit_to_exact_size(egui::Vec2 {
                                    x: adjusted_size.width,
                                    y: adjusted_size.height,
                                })
                                .sense(egui::Sense::click_and_drag()),
                            );

                            self.responses.insert(
                                view_label.clone(),
                                ResponseStruct {
                                    ui_response,
                                    scales: ViewportScale::from_image_size_and_viewport_size(
                                        view.intrinsics().image_size(),
                                        adjusted_size,
                                    ),
                                    z_image: render_result.depth_image.ndc_z_image,
                                    view_port_size: adjusted_size.image_size(),
                                },
                            );
                        }
                        View::Image(view) => {
                            let render_result = view
                                .renderer
                                .render_params(
                                    &adjusted_size.image_size(),
                                    &view.interaction.scene_from_camera(),
                                )
                                .zoom(view.interaction.zoom2d())
                                .interaction(view.interaction.marker())
                                .backface_culling(self.backface_culling)
                                .render();

                            let ui_response = ui.add(
                                egui::Image::new(egui::load::SizedTexture {
                                    size: egui::Vec2::new(
                                        adjusted_size.width,
                                        adjusted_size.height,
                                    ),
                                    id: render_result.rgba_egui_tex_id,
                                })
                                .fit_to_exact_size(egui::Vec2 {
                                    x: adjusted_size.width,
                                    y: adjusted_size.height,
                                })
                                .sense(egui::Sense::click_and_drag()),
                            );

                            self.responses.insert(
                                view_label.clone(),
                                ResponseStruct {
                                    ui_response,
                                    scales: ViewportScale::from_image_size_and_viewport_size(
                                        view.intrinsics().image_size(),
                                        adjusted_size,
                                    ),
                                    z_image: render_result.depth_image.ndc_z_image,
                                    view_port_size: adjusted_size.image_size(),
                                },
                            );
                        }
                    }
                }
            });
        });
    }

    pub(crate) fn add_renderables_to_tables(&mut self) {
        loop {
            let maybe_stream = self.message_recv.try_recv();
            if maybe_stream.is_err() {
                break;
            }
            let stream = maybe_stream.unwrap();
            for packet in stream.packets {
                match packet {
                    Packet::Scene(packet) => {
                        SceneView::update(&mut self.views, packet, &self.state)
                    }
                    Packet::Image(packet) => {
                        ImageView::update(&mut self.views, packet, &self.state)
                    }
                }
            }
        }
    }
}
