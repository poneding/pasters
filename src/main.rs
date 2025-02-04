use app::Pasters;
use core::time;
use eframe::{run_native, App, CreationContext, NativeOptions};
use egui::{
    epaint::text::{FontInsert, FontPriority, InsertFontFamily},
    FontData, FontFamily, ViewportBuilder, ViewportCommand,
};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use keyboard_types::{Code, Modifiers};
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    thread,
};

mod app;
mod clipboard;

fn main() -> Result<(), eframe::Error> {
    let (tx, clipboard_rx) = mpsc::channel();
    // 监听剪贴板
    clipboard::watch(tx);

    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);

    manager.register(hotkey).unwrap();
    let receiver = GlobalHotKeyEvent::receiver();
    let (hotkey_tx, hotkey_rx) = mpsc::channel();

    let cb = Pasters::new(clipboard_rx, hotkey_rx);

    thread::spawn(move || loop {
        for event in receiver.try_iter() {
            if event.state == HotKeyState::Pressed {
                println!("Hotkey pressed");
                hotkey_tx.send(true).unwrap();
            }
        }
        thread::sleep(time::Duration::from_millis(100));
    });

    run_pasters(cb)
}

fn run_pasters(cb: Pasters) -> Result<(), eframe::Error> {
    let options = NativeOptions {
        run_and_return: true,
        viewport: ViewportBuilder::default()
            .with_inner_size([380.0, 360.0])
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    run_native(
        "Pasters",
        options,
        Box::new(|ctx| {
            set_font(ctx);
            Ok(Box::<Pasters>::from(cb))
        }),
    )
}

fn set_font(cc: &CreationContext) {
    cc.egui_ctx.add_font(FontInsert::new(
        "custom-font",
        FontData::from_static(include_bytes!("../fonts/SarasaGothicSC-Regular.ttf")),
        vec![
            InsertFontFamily {
                family: FontFamily::Proportional,
                priority: FontPriority::Highest,
            },
            InsertFontFamily {
                family: FontFamily::Monospace,
                priority: FontPriority::Lowest,
            },
        ],
    ));
}
