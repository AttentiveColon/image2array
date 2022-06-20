use eframe::{egui, NativeOptions};
use emath::vec2;
use clipboard::{ClipboardProvider, ClipboardContext};
use image::io;

struct I2A {
    header_str: String,
    path_str: String,
    output_str: String,
    clipboard: ClipboardContext,
}

impl Default for I2A {
    fn default() -> Self {
        Self {
            header_str: "Path2Image".to_owned(),
            path_str: String::new(),
            output_str: String::new(),
            clipboard: ClipboardProvider::new().expect("Couldn't create clipboard context"),
        }
    }
}

impl eframe::App for I2A {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.header_str);
            ui.text_edit_singleline(&mut self.path_str);
            if ui.button("Generate").clicked() {
                let img = io::Reader::open(&self.path_str.trim_end_matches("\"").trim_start_matches("\""));
                match img {
                    Ok(image) => match image.decode() {
                        Ok(image) => self.output_str = format!("{:?}", image.as_bytes()),
                        Err(_) => self.output_str = format!("Decoding image failed"),
                    },
                    Err(_) => self.output_str = format!("Couldn't Resolve Path"),
                }
            }
            if self.output_str.len() > 50 {
                ui.text_edit_singleline(&mut format!("{} ...", &self.output_str[0..45]));
            } else {
                ui.text_edit_singleline(&mut self.output_str);
            }
            if ui.button("Copy to Clipboard").clicked() {
                self.clipboard.set_contents(self.output_str.clone()).expect("Couldn't copy contents to Clipboard");
            }
        });
    }
}

fn main() {
    let options = NativeOptions {
        resizable: false,
        initial_window_size: Some(vec2(320.0, 130.0)),
        ..NativeOptions::default()
    };
    eframe::run_native(
        "Image2Array",
        options,
        Box::new(|_cc| Box::new(I2A::default())),
    );
}
