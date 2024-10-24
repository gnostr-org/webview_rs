//#![windows_subsystem = "windows"]

extern crate tinyfiledialogs as tfd;
extern crate webview_rs;

use tfd::MessageBoxIcon;
use webview_rs::*;

fn main() -> WVResult {
    let webview = webview_rs::builder()
        .title("webview.rs")
        .content(Content::Html(HTML))
        .size(800, 666)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "open" => match tfd::open_file_dialog("Please choose a file...", "", None) {
                    Some(path) => tfd::message_box_ok("File chosen", &path, MessageBoxIcon::Info),
                    None => tfd::message_box_ok(
                        "Warning",
                        "You didn't choose a file.",
                        MessageBoxIcon::Warning,
                    ),
                },
                "save" => match tfd::save_file_dialog("Save file...", "") {
                    Some(path) => tfd::message_box_ok("File chosen", &path, MessageBoxIcon::Info),
                    None => tfd::message_box_ok(
                        "Warning",
                        "You didn't choose a file.",
                        MessageBoxIcon::Warning,
                    ),
                },
                "info" => {
                    tfd::message_box_ok("Info", "This is a info dialog", MessageBoxIcon::Info)
                }
                "warning" => tfd::message_box_ok(
                    "Warning",
                    "This is a warning dialog",
                    MessageBoxIcon::Warning,
                ),
                "error" => {
                    tfd::message_box_ok("Error", "This is a error dialog", MessageBoxIcon::Error)
                }
                "exit" => webview.exit(),
                _ => unimplemented!(),
            };
            Ok(())
        })
        .build()?;

    webview.run()
}

const HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>dialog.rs</title>
    <style>
li {
  float: left;
}
li a {
  display: block;
  padding: 8px;
  background-color: \#dddddd;
}
ul {
  list-style-type: none;
  margin: 0;
  padding: 0;
  overflow: hidden;
  background-color: #333;
  position: fixed;
  top: 0;
  width: 100%;
}
li {
  float: left;
}
li a {
  display: block;
  color: white;
  text-align: center;
  padding: 14px 16px;
  text-decoration: none;
}
li a:hover:not(.active) {
  background-color: #111;
}
.active {
  background-color: #04AA6D;
}
button {
    background: none;
    color: white;
    border: none;
    padding: 20px;
    font: inherit;
    cursor: pointer;
    outline: inherit;
}
button:focus {
  outline: revert;
}
body {

  overflow-y: hidden; /* Hide vertical scrollbar */
  /* overflow-x: hidden; /* Hide horizontal scrollbar */

  margin-top: 70px;
  margin-bottom: 0px;
  margin-right: 0px;
  margin-left: 0px;

}
.content {
  padding-top: 0px;
  padding-bottom: 10px;
  padding-right: 10px;
  padding-left: 10px;

}
</style>
</head>
<body>
<ul>
<li><button onclick="external.invoke('open')">Open</button></li>
<li><button onclick="external.invoke('save')">Save</button></li>
<li><button onclick="external.invoke('info')">Info</button></li>
<li><button onclick="external.invoke('warning')">Warning</button></li>
<li><button onclick="external.invoke('exit')">Exit</button></li>
</ul>

<div class="content">
<h1>Lorem Ipsum</h1>
<p>
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis nec venenatis nisi. Morbi eget massa rutrum, bibendum quam vitae, gravida felis. Praesent ac fringilla diam. Donec semper metus quis ipsum mollis gravida. Etiam ut odio dolor. Nullam vel sapien vel risus tristique iaculis. Integer rhoncus nunc lacus, eget luctus orci malesuada a. Nam sit amet nunc purus. Maecenas facilisis dapibus ultrices.
</p>
<p>
Maecenas a lacus facilisis, viverra tellus ac, ultrices lectus. Praesent tincidunt porta vehicula. Aliquam ex sapien, tempor at euismod eget, luctus ac nunc. Etiam finibus ex venenatis sem posuere sodales. Phasellus eget posuere est. Quisque elementum mi id libero faucibus, ut rutrum lacus egestas. Fusce tristique pulvinar diam vitae varius. Vivamus lectus velit, pharetra id imperdiet sit amet, sodales in est. Phasellus vitae pellentesque diam. Nam faucibus at ipsum eget lobortis. Vestibulum in libero posuere, molestie mi ut, euismod orci.
</p>
<p>
Nulla enim orci, ullamcorper sit amet congue id, commodo non purus. Integer urna odio, pulvinar quis metus porta, malesuada euismod libero. Maecenas varius tempus placerat. Morbi non turpis vel arcu dignissim placerat. Mauris rutrum faucibus suscipit. Duis at interdum lacus, ac maximus nisl. Maecenas libero ante, scelerisque nec venenatis eu, finibus vitae ligula. Quisque in tempor ipsum. Curabitur est tellus, porttitor ut rutrum sit amet, maximus pellentesque elit.
</p>
<p>
Nullam a metus eu magna auctor tempus. Fusce euismod efficitur nunc nec vestibulum. Nulla facilisi. Duis gravida tellus elit, vitae venenatis ligula porttitor ac. Nunc id justo aliquam, sagittis ex vel, pretium ante. Pellentesque mattis elit in erat facilisis, id vehicula ante mattis. Sed vel nunc vulputate, vehicula diam at, ornare sem. Cras tempor, elit tempus lacinia pretium, sem lectus faucibus eros, et volutpat risus metus ut arcu. Nunc a feugiat mauris. Vivamus sagittis interdum lacus et varius. Sed id odio magna. Nullam non ultricies tellus, blandit lobortis metus. Suspendisse eu congue eros. Curabitur placerat dui arcu, a dapibus est vulputate vel.
</p>
<p>
In et porttitor libero, quis sagittis tortor. Nunc non rhoncus eros, pretium accumsan eros. Fusce hendrerit nec tellus in accumsan. Aliquam vitae ipsum mattis, cursus lectus id, congue nulla. Cras consequat lorem lacus, sed faucibus massa ullamcorper sit amet. Etiam id orci sit amet augue laoreet dapibus vel vel leo. Duis non nibh in mi bibendum blandit pulvinar ac lorem. Integer commodo consequat finibus. Curabitur non aliquet nibh, at interdum erat. Nullam iaculis libero tellus, sit amet iaculis mi pharetra a. Morbi efficitur, ex eleifend placerat rhoncus, velit sem blandit tortor, sit amet facilisis metus nisi nec sem. Suspendisse neque libero, placerat vitae laoreet quis, tincidunt non neque.
</p>

</div>
</body>
</html>
"#;
