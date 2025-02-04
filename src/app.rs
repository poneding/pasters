use core::time;
use std::{
    collections::VecDeque,
    sync::{mpsc, Arc, Mutex},
    thread, vec,
};

use eframe::App;
use egui::{
    CentralPanel, ScrollArea, Sense, TextStyle, ViewportBuilder, ViewportCommand, ViewportId,
    WindowLevel,
};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use global_hotkey::{hotkey, GlobalHotKeyEvent, GlobalHotKeyEventReceiver, HotKeyState};

use crate::clipboard;

const VERSION: &str = "0.1.0";

pub(crate) struct Pasters {
    history: VecDeque<String>,
    selected_index: usize,
    pub(crate) visiable: Arc<Mutex<bool>>,
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    clipboard_rx: mpsc::Receiver<String>,
}

impl Pasters {
    pub(crate) fn new(
        clipboard_rx: mpsc::Receiver<String>,
        hotkey_rx: mpsc::Receiver<bool>,
    ) -> Self {
        let show_panel = Arc::new(Mutex::new(false));
        let show_panel_clone = show_panel.clone();

        thread::spawn(move || loop {
            for hotkey_event in hotkey_rx.try_iter() {
                println!("hotkey event: {:?}", hotkey_event);
                *show_panel_clone.lock().unwrap() = true;
            }
        });

        let instance = Self {
            history: VecDeque::new(),
            selected_index: 0,
            visiable: Arc::new(Mutex::new(true)),
            allowed_to_close: false,
            show_confirmation_dialog: false,
            clipboard_rx,
        };

        instance
    }
}

impl App for Pasters {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) && !self.allowed_to_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.show_confirmation_dialog = true;
        }

        for msg in self.clipboard_rx.try_iter() {
            if self.history.contains(&msg) {
                self.history.retain(|x| x != &msg);
            }
            println!("msg: {}", msg);
            self.history.push_front(msg);
        }

        // ctx.send_viewport_cmd(ViewportCommand::Minimized(false));
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
                            ui.horizontal(|ui| {
                                ui.label("Tips: (Esc to close, Enter to paste)");
                                ui.separator();
                                ui.label(format!("Version: v{}", VERSION));
                                ui.hyperlink_to("GitHub", "https://github.com/poneding/pasters");
                            });
                        });
                    });
                });

            // 处理上下键选择
            ui.input(|i| {
                if self.history.is_empty() {
                    return;
                }
                if i.key_pressed(egui::Key::ArrowUp) && self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            });
            ui.input(|i| {
                if self.history.is_empty() {
                    return;
                }

                if i.key_pressed(egui::Key::ArrowDown)
                    && self.selected_index < self.history.len() - 1
                {
                    self.selected_index += 1;
                }
            });

            // 处理 Enter 键
            ui.input(|i| {
                if i.key_pressed(egui::Key::Enter) {
                    if let Some(selected_text) = self.history.get(self.selected_index) {
                        clipboard::set_contents(selected_text);
                        self.visiable = Arc::new(Mutex::new(false));
                    }
                }
            });

            // 处理 Esc 键
            ui.input(|i| {
                if i.key_pressed(egui::Key::Escape) {
                    self.visiable = Arc::new(Mutex::new(false));
                }
            });

            // 处理 Delete 键
            ui.input(|i| {
                if i.key_pressed(egui::Key::Delete) {
                    self.history.remove(self.selected_index);
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                    }
                }
            });
        });

        if !*self.visiable.lock().unwrap() {
            self.visiable = Arc::new(Mutex::new(true));
            ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
        }

        // 处理关闭确认对话框
        if self.show_confirmation_dialog {
            CentralPanel::default().show(ctx, |ui| {
                ui.label("Do you want to quit?\n");
                ui.horizontal(|ui| {
                    if ui.button("No").highlight().clicked() {
                        self.show_confirmation_dialog = false;
                        self.allowed_to_close = false;
                    }

                    if ui.button("Yes").clicked() {
                        self.show_confirmation_dialog = false;
                        self.allowed_to_close = true;
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        }
    }
}

impl Pasters {
    fn ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();
        let table = TableBuilder::new(ui)
            // .striped(true)
            .resizable(false)
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

        let mut remove_index = vec![];
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
                        row.set_selected(self.selected_index == i);

                        row.col(|ui| {
                            ui.label(format!("{}.", i + 1));
                        });
                        row.col(|ui| {
                            ui.label(item);
                        });
                        row.col(|ui| {
                            if ui.button("x").clicked() {
                                remove_index.push(i);
                            }
                        });

                        let row_resp = row.response();

                        if row_resp.clicked() {
                            self.selected_index = i;
                            row.set_selected(true);
                        }

                        if row_resp.double_clicked() {
                            clipboard::set_contents(item);
                            self.visiable = Arc::new(Mutex::new(false));
                        }
                    });
                }
            });

        for i in remove_index.iter() {
            self.history.remove(*i);
        }
    }
}
