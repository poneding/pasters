use eframe::App;
use egui::{CentralPanel, ScrollArea, Sense, TextStyle};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

pub(crate) struct Pasters {
    history: Vec<String>,
    row_actived: usize,
}

impl Default for Pasters {
    fn default() -> Self {
        Self {
            history: vec!["Hello".to_string(), "World".to_string()],
            // current: String::new(),
            row_actived: 0,
        }
    }
}

impl App for Pasters {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.input(|i| {
        //     if i.key_down(Key::Enter) {
        //         println!("")
        //     }
        // });

        CentralPanel::default().show(ctx, |ui| {
            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            StripBuilder::new(ui)
                .size(egui_extras::Size::remainder().at_least(100.0))
                .size(Size::exact(body_text_size))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ScrollArea::horizontal().show(ui, |ui| {
                            self.ui(ctx, ui);
                        });
                    });

                    strip.cell(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Tips: (Esc to close, Enter to paste)");
                        });
                    });
                });
        });
    }
}

impl Pasters {
    pub(crate) fn watch_clipboard(&mut self) {}

    pub(crate) fn watch_shortcut(&mut self) {}

    fn ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().resizable(false))
            .column(
                Column::remainder()
                    .at_least(40.0)
                    .clip(true)
                    .resizable(false),
            )
            .column(Column::auto())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height)
            .sense(Sense::click());

        // table = table.sense(egui::Sense::click());

        table
            // .header(20.0, |mut header| {
            //     header.col(|ui| {
            //         ui.strong("#");
            //     });
            //     header.col(|ui| {
            //         ui.strong("Content");
            //     });
            //     header.col(|ui| {
            //         ui.strong("Interaction");
            //     });
            // })
            .body(|mut body| {
                for (i, item) in self.history.iter().enumerate() {
                    body.row(text_height, |mut row| {
                        row.set_hovered(false);
                        row.set_selected(self.row_actived == i);

                        row.col(|ui| {
                            ui.label(format!("{}.", i + 1));
                        });
                        row.col(|ui| {
                            ui.label(item);
                        });
                        row.col(|ui| {
                            if ui.button("x").clicked() {
                                // Todo: Remove item
                                println!("Remove {}", i);
                            }
                        });

                        if row.response().clicked() {
                            self.row_actived = i;
                            row.set_selected(true);
                        }
                    });
                }
            });
    }
}
