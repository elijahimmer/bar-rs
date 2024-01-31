pub mod clock;

use gtk::prelude::*;
use gtk::{glib, Align, Application, Button, Calendar, Window};
use gtk_layer_shell::LayerShell;
use std::time::Duration;

pub fn new(app: Application) -> Button {
    log::trace!("Initalizing Time Widget");

    let clock = crate::time::clock::Clock::new();

    let clock_button = Button::builder()
        .child(&clock.container)
        .valign(Align::Center)
        .halign(Align::Center)
        .name("calendar")
        .hexpand(false)
        .name("clock")
        .build();

    let calender = Calendar::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    let window = Window::builder()
        .application(&app)
        .title("bar-rs_calender")
        .name("calendar")
        .decorated(false)
        .resizable(false)
        .halign(Align::Center)
        .valign(Align::Center)
        .child(&calender)
        .build();

    window.init_layer_shell();
    window.set_anchor(gtk_layer_shell::Edge::Top, true);

    clock_button.connect_clicked(move |_button| {
        match glib::DateTime::now_local() {
            Ok(date) => calender.select_day(&date),
            Err(err) => log::warn!("Failed to get current date! error={err}"),
        };

        let visible = window.get_visible();
        if visible {
            log::info!("Calender Hidden");
        } else {
            log::info!("Calender Presenting");
        }
        window.set_visible(!visible);
    });

    glib::timeout_add_local(Duration::from_millis(250), move || {
        match clock.update() {
            Ok(()) => {}
            Err(err) => {
                log::warn!("Clock failed to update time. error={err}")
            }
        };

        glib::ControlFlow::Continue
    });

    clock_button
}
