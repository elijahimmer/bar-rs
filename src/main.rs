pub mod battery;
pub mod brightness;
pub mod cpu;
pub mod css;
pub mod ram;
pub mod time;
pub mod updated_last;
pub mod utils;
pub mod volume;
pub mod workspaces;

use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Align};
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

    application.add_main_option(
        "updated-last",
        b'U'.into(),
        glib::OptionFlags::OPTIONAL_ARG,
        glib::OptionArg::Int64,
        "Unix timestamp of last update to system",
        None,
    );

    application.connect_activate(build_ui);

    application.run()
}

fn build_ui(app: &Application) {
    log::info!("Building UI");
    let end_box = gtk::Box::builder()
        .name("end-box").build();

    log::trace!("Initalizing Widgets:");
    append_res!(end_box; app; updated_last, cpu, ram, volume, brightness, battery);

    let start_wgt = match workspaces::element() {
        Ok(a) => a,
        Err(err) => {
            log::warn!("Workspace Widget Disabled. error={err}");

            Default::default()
        }
    };

    let center_wgt = time::new(app.clone());

    log::trace!("Initalizing Window");
    let main_box = gtk::CenterBox::builder()
        .start_widget(&start_wgt)
        .center_widget(&center_wgt)
        .end_widget(&end_box)
        .valign(Align::Center)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .child(&main_box)
        .default_height(25)
        .title("bar-rs")
        .name("main")
        .valign(Align::Center)
        .build();

    log::trace!("Starting Layer Shell.");
    window.init_layer_shell();
    window.auto_exclusive_zone_enable();
    window.set_anchor(gtk_layer_shell::Edge::Right, true);
    window.set_anchor(gtk_layer_shell::Edge::Left, true);
    window.set_anchor(gtk_layer_shell::Edge::Top, true);
    window.set_anchor(gtk_layer_shell::Edge::Bottom, false);

    log::info!("Window Presenting");
    window.present();

    gtk::style_context_add_provider_for_display(
        &match Display::default() {
            Some(val) => val,
            None => {
                log::error!("display now found!");
                return;
            }
        },
        &css::css(),
        0,
    );
}
