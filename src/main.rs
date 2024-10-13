use eframe::*;
use egui::{CentralPanel, Response, Ui};

struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("hello world");

            let button_response: Response = ui.button("click");
            if button_response.clicked() {
                println!("clicked!");
            }
        });
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    run_native(
        "hello-egui", 
        NativeOptions::default(), 
        Box::new(|_cc: &CreationContext<'_>| {
            Ok(Box::new(MyApp {}))
        })
    )
}
