use eframe::{egui, NativeOptions};
use emath::vec2;
use clipboard::{ClipboardProvider, ClipboardContext};

struct I2A {
    text: String,
    my_str: String,
    output: String,
    clipboard: ClipboardContext,
}

impl Default for I2A {
    fn default() -> Self {
        Self {
            text: "Path2Image".to_owned(),
            my_str: String::new(),
            output: String::new(),
            clipboard: ClipboardProvider::new().expect("Couldn't create clipboard context"),
        }
    }
}

impl eframe::App for I2A {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.text);
            ui.text_edit_singleline(&mut self.my_str);
            if ui.button("Generate").clicked() {
                let img = image::io::Reader::open(self.my_str.clone()).unwrap().decode().unwrap();
                self.output = format!("{:?}", img.as_bytes());
            }
            ui.text_edit_singleline(&mut self.output);
            if ui.button("Copy to Clipboard").clicked() {
                self.clipboard.set_contents(self.output.clone()).expect("Couldn't copy contents to Clipboard");
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
