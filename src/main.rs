pub mod battery;
pub mod brightness;
pub mod clock;
pub mod cpu;
pub mod css;
pub mod ram;
pub mod util;
pub mod volume;
pub mod workspaces;
//pub mod wttr;

use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk_layer_shell::LayerShell;

fn main() -> glib::ExitCode {
    env_logger::Builder::from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    )
    .format_timestamp_millis()
    .init();

    let application = Application::builder()
        .application_id("me.eimmer.bar-rs")
        .build();

    log::info!("Building UI");
    application.connect_activate(build_ui);

    application.run()
}

fn build_ui(app: &Application) {
    log::trace!("Initalizing Components");

    let end_box = gtk::Box::builder().name("end-box").build();

    if let Some(b) = cpu::element() {
        end_box.append(&b);
    }

    if let Some(b) = ram::element() {
        end_box.append(&b);
    }

    if let Some(b) = volume::element() {
        end_box.append(&b);
    }

    if let Some(b) = brightness::element() {
        end_box.append(&b);
    };

    if let Some(b) = battery::element() {
        end_box.append(&b);
    };

    let start_wgt = match workspaces::element() {
        Ok(w) => w,
        Err(e) => {
            log::error!("Failed to create workspaces widget: {e}");

            gtk::Box::builder().build()
        }
    };

    let main_box = gtk::CenterBox::builder()
        .start_widget(&start_wgt)
        .center_widget(&clock::element())
        .end_widget(&end_box)
        .build();

    log::trace!("Initalizing window");
    let window = ApplicationWindow::builder()
        .application(app)
        .title("bar-rs")
        .decorated(false)
        .default_height(25)
        .child(&main_box)
        .build();

    window.init_layer_shell();
    window.auto_exclusive_zone_enable();
    window.set_anchor(gtk_layer_shell::Edge::Right, true);
    window.set_anchor(gtk_layer_shell::Edge::Left, true);
    window.set_anchor(gtk_layer_shell::Edge::Top, true);
    window.set_anchor(gtk_layer_shell::Edge::Bottom, false);

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Display not found."),
        &css::css(),
        0,
    );

    log::info!("Window presenting");
    window.present();
}
