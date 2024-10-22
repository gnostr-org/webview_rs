#![windows_subsystem = "windows"]

extern crate webview_rs;

use webview_rs::*;

fn main() {
    webview_rs::builder()
        .title("Minimal webview example")
        .content(Content::Url("https://en.m.wikipedia.org/wiki/Main_Page"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
