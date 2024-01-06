use gtk::prelude::*;
use gtk::{glib, Align, Application, Box, Button, Calendar, Label, Window};
use gtk_layer_shell::LayerShell;
use std::time::Duration;

pub fn element(app: Application) -> Button {
    let hours_label = Label::new(None);
    let minutes_label = Label::new(None);
    let seconds_label = Label::new(None);

    // I cannot find a better way to copy over this element.
    // I may want to look into glib::clone!(), but that may just be more
    //      than what is needed here.
    let spacing1 = Label::builder().css_classes(["spacer"]).label("").build();

    let spacing2 = Label::builder().css_classes(["spacer"]).label("").build();

    let clock_box = Box::new(gtk::Orientation::Horizontal, 0);

    clock_box.append(&hours_label);
    clock_box.append(&spacing1);
    clock_box.append(&minutes_label);
    clock_box.append(&spacing2);
    clock_box.append(&seconds_label);

    let clock_button = Button::builder()
        .child(&clock_box)
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

    log::trace!("Initalizing Clock Window");
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
        let visible = window.get_visible();
        if visible {
            log::info!("Calender Hidden");
        } else {
            log::info!("Calender Presenting");
        }
        window.set_visible(!visible);
    });

    glib::timeout_add_local(Duration::from_millis(250), move || {
        let now = chrono::Local::now();

        hours_label.set_label(&now.format("%H").to_string());
        minutes_label.set_label(&now.format("%M").to_string());
        seconds_label.set_label(&now.format("%S").to_string());

        glib::ControlFlow::Continue
    });

    clock_button
}
