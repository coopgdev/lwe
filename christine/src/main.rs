use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use std::process::Command;

fn main() {
    let app = Application::builder()
        .application_id("com.greenec03.christine")
        .build();
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Chromium")
        .margin_bottom(12)
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| {
        let _ = Command::new("swaymsg")
            .arg("exec chromium")
            .spawn();
    });

    let window = ApplicationWindow::builder()
        .title("Test")
        .application(app)
        .child(&button)
        .build();

    window.show();
}
