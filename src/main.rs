use app::Pasters;
use eframe::{run_native, NativeOptions};
use egui::ViewportBuilder;

mod app;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([380.0, 360.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_minimize_button(false)
            .with_close_button(false),
        ..Default::default()
    };

    let mut cb = Pasters::default();

    cb.watch_clipboard();
    cb.watch_shortcut();

    run_native(
        "Pasters",
        options,
        Box::new(|_cc| Ok(Box::<Pasters>::from(cb))),
    )
}
