#![windows_subsystem = "windows"]

extern crate webview_rs;

use webview_rs::*;

fn main() {
    webview_rs::builder()
        .title("Rust / Elm - Counter App")
        .content(Content::Html(include_str!("elm-counter/index.html")))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
