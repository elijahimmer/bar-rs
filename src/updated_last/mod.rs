pub mod clock;

use anyhow::Result;
use std::time::SystemTime;
use gtk::{glib, Align, Application, Button, Label};

pub fn new(_app: Application) -> Result<Label> {
    log::trace!("Initalizing Updated Last Widget");


    let contents = match fs::read_to_string(UPDATE_PATH) {
        Ok(date) => date,
        Err(err) => {
            log::warn!("Couldn't read file. error={err}");
            "".to_string()
        }
    };


    let label = Label::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .build();

    label.set_text(content.trim());

    glib::timeout_add_seconds_local(1, move || {

        glib::ControlFlow::Continue
    });

    Ok(label)
}
