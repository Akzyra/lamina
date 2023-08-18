#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui::{CentralPanel, Frame, TopBottomPanel};

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 400.0)),
        min_window_size: Some(egui::vec2(400.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "lamina",
        native_options,
        Box::new(|cc| Box::new(LaminaApp::new(cc))),
    )
}

struct LaminaApp {}

impl LaminaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for LaminaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top")
            .frame(Frame::central_panel(&ctx.style()))
            .show(ctx, |ui| {
                ui.heading("lamina")
            });

        CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()))
            .show(ctx, |ui| {
                let mut todo = "todo\nadd UI";
                ui.text_edit_multiline(&mut todo);
            });
    }
}