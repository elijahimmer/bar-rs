use gtk::prelude::*;
use gtk::{glib, Align, Box, Button, Label};
use std::time::Duration;

pub fn element() -> Button {
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
        .hexpand(false)
        .name("clock")
        .build();

    glib::timeout_add_local(Duration::from_millis(250), move || {
        let now = chrono::Local::now();

        hours_label.set_label(&now.format("%H").to_string());
        minutes_label.set_label(&now.format("%M").to_string());
        seconds_label.set_label(&now.format("%S").to_string());

        glib::ControlFlow::Continue
    });

    clock_button
}
