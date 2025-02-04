#![allow(unused)]
use clipboard::{ClipboardContext, ClipboardProvider};
use std::{
    sync::mpsc::{self},
    thread::{self, sleep},
};

fn new_clipboard() -> ClipboardContext {
    ClipboardProvider::new().unwrap()
}

pub(crate) fn get_contents() -> String {
    let mut cc = new_clipboard();
    cc.get_contents().unwrap()
}

pub(crate) fn set_contents(text: &str) {
    let mut cc = new_clipboard();
    cc.set_contents(text.to_owned()).unwrap();
}

pub(crate) fn watch(tx: mpsc::Sender<String>) {
    thread::spawn(move || {
        let mut cc = new_clipboard();
        let mut last_text = cc.get_contents().unwrap();
        loop {
            let text = cc.get_contents().unwrap();
            if last_text != text {
                tx.send(text.clone()).unwrap();
                last_text = text;
            }
            sleep(std::time::Duration::from_millis(100));
        }
    });
}
