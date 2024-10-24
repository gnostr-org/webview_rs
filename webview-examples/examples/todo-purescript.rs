#![windows_subsystem = "windows"]

extern crate webview_rs;

use webview_rs::*;

fn main() {
    webview_rs::builder()
        .title("Rust / PureScript - Todo App")
        .content(Content::Html(include_str!("todo-ps/dist/bundle.html")))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
