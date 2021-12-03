use eframe::{egui::CentralPanel, epi::App, run_native, egui::Ui};
use eframe::NativeOptions;

struct Window {  }

impl App for Window {
    fn update(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
    ) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Presence GUI");
        });
    }
    fn name(&self) -> &str {
        "Rich Presence"
    }
} 

fn main() {
    let app = Window{  };
    let native_options = NativeOptions::default();
    run_native(Box::new(app), native_options);
}
