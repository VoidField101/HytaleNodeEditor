use crate::app::HyNodeEditor;

mod editor;
mod errors;
mod generator;
mod workspace;
mod app;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 900.0]),

        ..Default::default()
    };
    eframe::run_native(
        "Open Hytale Nodeeditor",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(HyNodeEditor::new(cc)))
        }),
    )
}