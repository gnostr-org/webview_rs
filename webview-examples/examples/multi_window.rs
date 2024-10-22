#![windows_subsystem = "windows"]

extern crate webview_rs;

use webview_rs::*;

fn main() {
    let mut webview1 = webview_rs::builder()
        .title("Multi window example - Window 1")
        .content(Content::Url("https://en.m.wikipedia.org/wiki/Main_Page"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    let mut webview2 = webview_rs::builder()
        .title("Multi window example - Window 2")
        .content(Content::Url("https://en.m.wikipedia.org/wiki/Main_Page"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    loop {
        if webview1.step().is_none() {
            break;
        }

        if webview2.step().is_none() {
            break;
        }
    }
}
