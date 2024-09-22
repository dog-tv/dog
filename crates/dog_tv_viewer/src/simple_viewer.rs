use crate::viewer_base::ViewerBase;
use dog_tv_renderer::renderables::Packets;
use dog_tv_renderer::RenderContext;
use eframe::egui;

/// Simple viewer
pub struct SimpleViewer {
    base: ViewerBase,
}

impl SimpleViewer {
    /// Create a new simple viewer
    pub fn new(
        render_state: RenderContext,
        message_recv: std::sync::mpsc::Receiver<Packets>,
    ) -> Box<SimpleViewer> {
        Box::new(SimpleViewer {
            base: ViewerBase::new(render_state, message_recv),
        })
    }
}

impl eframe::App for SimpleViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.base.update_data();

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            self.base.bottom_status_bar(ui);
        });

        egui::SidePanel::left("left").show(ctx, |ui| {
            self.base.update_left_panel(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.base.update_central_panel(ui, ctx);
        });

        self.base.process_events();

        ctx.request_repaint();
    }
}
