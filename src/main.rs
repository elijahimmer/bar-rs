use gtk4 as gtk;
extern crate gtk4_layer_shell as gtk_layer_shell;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use gtk4_layer_shell::LayerShell;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("me.eimmer.bar-rs")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("bar-rs")
            .decorated(false)
            .default_height(25)
            .build();

        window.init_layer_shell();
        window.auto_exclusive_zone_enable();
        window.set_anchor(gtk_layer_shell::Edge::Right, true);
        window.set_anchor(gtk_layer_shell::Edge::Left, true);
        window.set_anchor(gtk_layer_shell::Edge::Top, true);
        window.set_anchor(gtk_layer_shell::Edge::Bottom, false);

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        window.present();
    });

    application.run()
}
