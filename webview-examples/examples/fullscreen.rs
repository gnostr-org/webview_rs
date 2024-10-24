extern crate webview_rs;

use webview_rs::*;

fn main() {
    webview_rs::builder()
        .title("Fullscreen example")
        .content(Content::Html(HTML))
        .size(800, 100)
        .resizable(true)
        .debug(true)
        .user_data("")
        .invoke_handler(|webview, arg| {
            match arg {
                "enter" => webview.set_fullscreen(true),
                "exit" => webview.set_fullscreen(false),
                _ => (),
            }
            Ok(())
        })
        .run()
        .unwrap();
}

const HTML: &str = r#"
<!doctype html>
<html>
	<body>
        <button onclick="external.invoke('enter')">enter fullscreen</button>
        <button onclick="external.invoke('exit')">exit fullscreen</button>
	</body>
</html>
"#;
